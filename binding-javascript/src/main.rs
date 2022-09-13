use wasmedge_quickjs::*;

fn main() {
    let mut ctx = Context::new();
    let code = r#"print('hello quickjs')"#;
    let r = ctx.eval_global_str(code);
    println!("return value:{:?}", r);
}