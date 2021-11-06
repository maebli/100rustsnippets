use std::cmp::max;
pub type CodingRate=u32;

pub enum CodingRates{
    CodingRate45,
    CodingRate46,
    CodingRate47,
    CodingRate48
}

pub struct Modulation {
    pub(crate) spreading_factor:u64,
    pub(crate) bandwidth:u64
}

pub struct FrameMetaInformation {
    pub(crate) payload_length:u64,
    pub(crate) preamble_length:u64,
    pub(crate) is_header_enabled:bool,
    pub(crate) is_low_data_rate_optimization_enabled:bool,
    pub(crate) modulation:Modulation,
    pub(crate) coding_rate:CodingRates,

}

impl CodingRates{
    fn value(&self) -> CodingRate {
        match self{
            CodingRates::CodingRate45 => 1,
            CodingRates::CodingRate46 => 2,
            CodingRates::CodingRate47 => 3,
            CodingRates::CodingRate48 => 4,
        }
    }
}

impl FrameMetaInformation{

    pub(crate) fn get_airtime(&self) -> u64{
        (self.get_preamble_duration() +
            self.modulation.get_symbol_duration() * self.get_payload_symbol_count() as f64) as u64
    }

    fn get_preamble_duration(&self) -> f64{
        self.modulation.get_preamble_duration(self.preamble_length)
    }

    fn get_payload_symbol_count(&self) -> u64 {
        let de = self.is_low_data_rate_optimization_enabled as u64;
        let h = (!self.is_low_data_rate_optimization_enabled) as u64;

        let a = (8 * self.payload_length + 28 + 16 - 20 * h) as f64;
        let b = (4 * (self.modulation.spreading_factor - 2 * de)) as f64;
        let c = (self.coding_rate.value() as f64) + 4.0;

        8 + max(((a / b).ceil() * c) as u64, 0)
    }

}

impl Modulation{
    fn get_symbol_duration(&self) -> f64{
        ((1<<self.spreading_factor) * 1000_000 / self.bandwidth) as f64
    }
    fn get_preamble_duration(&self,preamble_length:u64) -> f64{
        (100.0 * preamble_length as f64 + 425.0) / 100.0 * self.get_symbol_duration()
    }
}

#[test]
fn test_get_payload_symbol_count(){
    let f = FrameMetaInformation{
        payload_length: 13,
        preamble_length: 0,
        is_header_enabled: true,
        is_low_data_rate_optimization_enabled: false,
        modulation: Modulation {
            spreading_factor: 12,
            bandwidth: 0
        },
        coding_rate: CodingRates::CodingRate45
    };

    assert_eq!(23,f.get_payload_symbol_count())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_preamble_duration() {
        let m = Modulation {
            spreading_factor: 12,
            bandwidth: 125
        };
        assert_eq!(401408_000.0, m.get_preamble_duration(8));
    }

    #[test]
    fn test_get_symbol_duration() {
        let mut m = Modulation {
            spreading_factor: 12,
            bandwidth: 125
        };
        assert_eq!(32768_000.0, m.get_symbol_duration());
        m.spreading_factor = 9;
        assert_eq!(4096_000.0, m.get_symbol_duration());
        m.bandwidth = 500;
        assert_eq!(1024_000.0, m.get_symbol_duration());
    }

    #[test]
    fn test_airtime_calculation() {
        let f = FrameMetaInformation {
            payload_length: 13,
            preamble_length: 8,
            is_header_enabled: true,
            is_low_data_rate_optimization_enabled: false,
            modulation: Modulation {
                spreading_factor: 12,
                bandwidth: 125
            },
            coding_rate: CodingRates::CodingRate45,
        };

        assert_eq!(1_155_072_000, f.get_airtime() as u64);
    }
}