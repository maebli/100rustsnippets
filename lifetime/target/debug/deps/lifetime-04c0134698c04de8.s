	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 7
	.file	1 "/Volumes/data/4_github/learn-rust/100-rust-snippets/lifetime/src/main.rs"
	.p2align	4, 0x90
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hcfe3b45c960fa275E:
Lfunc_begin0:
	.file	2 "/rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs"
	.loc	2 121 0
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$32, %rsp
	movq	%rdi, -24(%rbp)
Ltmp3:
	.loc	2 125 18 prologue_end
	callq	__ZN4core3ops8function6FnOnce9call_once17h3e982e1eb2657badE
Ltmp0:
Ltmp4:
	.loc	2 128 5
	callq	__ZN4core4hint9black_box17h7866d660ef3e8471E
Ltmp1:
	jmp	LBB0_2
Ltmp5:
LBB0_2:
	.loc	2 131 2
	addq	$32, %rsp
	popq	%rbp
	retq
LBB0_3:
	.loc	2 131 1 is_stmt 0
	jmp	LBB0_5
LBB0_4:
Ltmp2:
	.loc	2 0 1
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, -16(%rbp)
	movl	%eax, -8(%rbp)
	jmp	LBB0_3
LBB0_5:
	.loc	2 121 1 is_stmt 1
	movq	-16(%rbp), %rdi
	callq	__Unwind_Resume
Ltmp6:
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table0:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Lfunc_begin0-Lfunc_begin0
	.uleb128 Ltmp0-Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp1
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__ZN3std2rt10lang_start17h172c3abe4c50711bE
	.globl	__ZN3std2rt10lang_start17h172c3abe4c50711bE
	.p2align	4, 0x90
__ZN3std2rt10lang_start17h172c3abe4c50711bE:
Lfunc_begin1:
	.file	3 "/rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/rt.rs"
	.loc	3 57 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, -24(%rbp)
	movq	%rdx, -16(%rbp)
	movq	%rcx, -8(%rbp)
Ltmp7:
	.loc	3 63 10 prologue_end
	movq	%rdi, -32(%rbp)
	.loc	3 63 9 is_stmt 0
	leaq	-32(%rbp), %rdi
	.loc	3 62 5 is_stmt 1
	leaq	l___unnamed_1(%rip), %rsi
	callq	__ZN3std2rt19lang_start_internal17hf65df31c3ffe3b9aE
	movq	%rax, -40(%rbp)
	.loc	3 0 5 is_stmt 0
	movq	-40(%rbp), %rdi
	.loc	3 62 5
	callq	__ZN4core6result19Result$LT$T$C$E$GT$7into_ok17h71d4d9dd8f81654fE
	movq	%rax, -48(%rbp)
	.loc	3 0 5
	movq	-48(%rbp), %rax
	.loc	3 68 2 is_stmt 1
	addq	$48, %rsp
	popq	%rbp
	retq
Ltmp8:
Lfunc_end1:
	.cfi_endproc

	.p2align	4, 0x90
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3f611c7bc2abae17E:
Lfunc_begin2:
	.loc	3 63 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
Ltmp9:
	.loc	3 63 77 prologue_end
	movq	(%rdi), %rdi
	.loc	3 63 18 is_stmt 0
	callq	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hcfe3b45c960fa275E
	callq	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h4e8c5194f038480cE
	movl	%eax, -12(%rbp)
	.loc	3 0 18
	movl	-12(%rbp), %eax
	.loc	3 63 91
	addq	$16, %rsp
	popq	%rbp
	retq
Ltmp10:
Lfunc_end2:
	.cfi_endproc

	.p2align	4, 0x90
__ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h5c510ae19c3bfc1aE:
Lfunc_begin3:
	.file	4 "/rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys/unix/process/process_common.rs"
	.loc	4 468 0 is_stmt 1
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rax
	movq	%rdi, -8(%rbp)
Ltmp11:
	.loc	4 469 9 prologue_end
	movzbl	(%rdi), %eax
	.loc	4 470 6
	addq	$8, %rsp
	popq	%rbp
	retq
Ltmp12:
Lfunc_end3:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h5d55519c156b3c13E:
Lfunc_begin4:
	.file	5 "/rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/ops/function.rs"
	.loc	5 227 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$32, %rsp
	movq	%rdi, -8(%rbp)
Ltmp13:
	.loc	5 227 5 prologue_end
	movq	(%rdi), %rdi
	callq	__ZN4core3ops8function6FnOnce9call_once17h0a4b30c31cd99dbfE
	movl	%eax, -20(%rbp)
	.loc	5 0 5 is_stmt 0
	movl	-20(%rbp), %eax
	.loc	5 227 5
	addq	$32, %rsp
	popq	%rbp
	retq
Ltmp14:
Lfunc_end4:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce9call_once17h0a4b30c31cd99dbfE:
Lfunc_begin5:
	.loc	5 227 0 is_stmt 1
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception1
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, -32(%rbp)
Ltmp15:
	leaq	-32(%rbp), %rdi
Ltmp18:
	.loc	5 227 5 prologue_end
	callq	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3f611c7bc2abae17E
Ltmp16:
	movl	%eax, -36(%rbp)
	jmp	LBB5_1
LBB5_1:
	jmp	LBB5_5
LBB5_2:
	jmp	LBB5_4
LBB5_3:
Ltmp17:
	.loc	5 0 5 is_stmt 0
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, -16(%rbp)
	movl	%eax, -8(%rbp)
	jmp	LBB5_2
LBB5_4:
	.loc	5 227 5
	movq	-16(%rbp), %rdi
	callq	__Unwind_Resume
LBB5_5:
	.loc	5 0 5
	movl	-36(%rbp), %eax
	.loc	5 227 5
	addq	$48, %rsp
	popq	%rbp
	retq
Ltmp19:
Lfunc_end5:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table5:
Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp15-Lfunc_begin5
	.uleb128 Ltmp16-Ltmp15
	.uleb128 Ltmp17-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp16-Lfunc_begin5
	.uleb128 Lfunc_end5-Ltmp16
	.byte	0
	.byte	0
Lcst_end1:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce9call_once17h3e982e1eb2657badE:
Lfunc_begin6:
	.loc	5 227 0 is_stmt 1
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
Ltmp20:
	.loc	5 227 5 prologue_end
	callq	*%rdi
	addq	$16, %rsp
	popq	%rbp
	retq
Ltmp21:
Lfunc_end6:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0a406dacefb1fee8E:
Lfunc_begin7:
	.file	6 "/rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/ptr/mod.rs"
	.loc	6 188 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rax
	movq	%rdi, -8(%rbp)
Ltmp22:
	.loc	6 188 1 prologue_end
	addq	$8, %rsp
	popq	%rbp
	retq
Ltmp23:
Lfunc_end7:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core4hint9black_box17h7866d660ef3e8471E:
Lfunc_begin8:
	.file	7 "/rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/hint.rs"
	.loc	7 159 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rax
Ltmp24:
	.loc	7 169 9 prologue_end
	## InlineAsm Start
	## InlineAsm End
	.loc	7 171 2
	addq	$8, %rsp
	popq	%rbp
	retq
Ltmp25:
Lfunc_end8:
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core6result19Result$LT$T$C$E$GT$7into_ok17h71d4d9dd8f81654fE:
Lfunc_begin9:
	.file	8 "/rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/result.rs"
	.loc	8 1424 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$32, %rsp
	movq	%rdi, -24(%rbp)
	movb	$1, %al
Ltmp26:
	.loc	8 1425 9 prologue_end
	testb	%al, %al
	jne	LBB9_2
	jmp	LBB9_4
LBB9_4:
	jmp	LBB9_3
	.loc	8 1425 15 is_stmt 0
	ud2
LBB9_2:
	.loc	8 1426 16 is_stmt 1
	movq	-24(%rbp), %rax
	movq	%rax, -8(%rbp)
	.loc	8 1429 6
	addq	$32, %rsp
	popq	%rbp
	retq
LBB9_3:
Ltmp27:
	.loc	8 1427 23
	callq	__ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h3492721134b3c734E
Ltmp28:
Lfunc_end9:
	.cfi_endproc

	.p2align	4, 0x90
__ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h4a35e1e720d56b42E:
Lfunc_begin10:
	.file	9 "/rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/convert/mod.rs"
	.loc	9 547 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rax
Ltmp29:
	.loc	9 549 6 prologue_end
	ud2
Ltmp30:
Lfunc_end10:
	.cfi_endproc

	.p2align	4, 0x90
__ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h3492721134b3c734E:
Lfunc_begin11:
	.loc	9 539 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
Ltmp31:
	.loc	9 540 9 prologue_end
	callq	__ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h4a35e1e720d56b42E
	.loc	9 541 6
	ud2
Ltmp32:
Lfunc_end11:
	.cfi_endproc

	.p2align	4, 0x90
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h4e8c5194f038480cE:
Lfunc_begin12:
	.file	10 "/rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/process.rs"
	.loc	10 2023 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
Ltmp33:
	.loc	10 2024 9 prologue_end
	xorl	%edi, %edi
	callq	__ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17hf74eeb29a67faa5bE
	movl	%eax, -12(%rbp)
	.loc	10 0 9 is_stmt 0
	movl	-12(%rbp), %eax
	.loc	10 2025 6 is_stmt 1
	addq	$16, %rsp
	popq	%rbp
	retq
Ltmp34:
Lfunc_end12:
	.cfi_endproc

	.p2align	4, 0x90
__ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17hf74eeb29a67faa5bE:
Lfunc_begin13:
	.loc	10 2057 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movb	%dil, %al
	movb	%al, -1(%rbp)
Ltmp35:
	.loc	10 2058 9 prologue_end
	leaq	-1(%rbp), %rdi
	callq	__ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h5c510ae19c3bfc1aE
	movl	%eax, -8(%rbp)
	.loc	10 0 9 is_stmt 0
	movl	-8(%rbp), %eax
	.loc	10 2059 6 is_stmt 1
	addq	$16, %rsp
	popq	%rbp
	retq
Ltmp36:
Lfunc_end13:
	.cfi_endproc

	.p2align	4, 0x90
__ZN8lifetime11getInstance17hcb8115af624794f7E:
Lfunc_begin14:
	.loc	1 5 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
Ltmp37:
	.loc	1 7 2 prologue_end
	leaq	__ZN8lifetime5THING17h57d16ec768adf029E(%rip), %rax
	popq	%rbp
	retq
Ltmp38:
Lfunc_end14:
	.cfi_endproc

	.p2align	4, 0x90
__ZN8lifetime4main17h6ed9b53b0ea9bc36E:
Lfunc_begin15:
	.loc	1 9 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$128, %rsp
Ltmp39:
	.loc	1 20 18 prologue_end
	leaq	l___unnamed_2(%rip), %rax
	movq	%rax, -128(%rbp)
	movq	$1, -120(%rbp)
Ltmp40:
	.loc	1 21 18
	leaq	l___unnamed_2(%rip), %rax
	movq	%rax, -112(%rbp)
	movq	$1, -104(%rbp)
Ltmp41:
	.loc	1 22 18
	leaq	l___unnamed_2(%rip), %rax
	movq	%rax, -96(%rbp)
	movq	$1, -88(%rbp)
Ltmp42:
	.loc	1 30 32
	leaq	l___unnamed_2(%rip), %rax
	movq	%rax, -80(%rbp)
	movq	$1, -72(%rbp)
Ltmp43:
	.loc	1 36 18
	leaq	l___unnamed_2(%rip), %rax
	movq	%rax, -64(%rbp)
	movq	$1, -56(%rbp)
Ltmp44:
	.loc	1 37 18
	leaq	l___unnamed_2(%rip), %rax
	movq	%rax, -48(%rbp)
	movq	$1, -40(%rbp)
Ltmp45:
	.loc	1 38 18
	leaq	l___unnamed_2(%rip), %rax
	movq	%rax, -32(%rbp)
	movq	$1, -24(%rbp)
Ltmp46:
	.loc	1 46 18
	callq	__ZN8lifetime11getInstance17hcb8115af624794f7E
	movq	%rax, -16(%rbp)
Ltmp47:
	.loc	1 47 18
	callq	__ZN8lifetime11getInstance17hcb8115af624794f7E
	movq	%rax, -8(%rbp)
Ltmp48:
	.loc	1 50 2
	addq	$128, %rsp
	popq	%rbp
	retq
Ltmp49:
Lfunc_end15:
	.cfi_endproc

	.globl	_main
	.p2align	4, 0x90
_main:
Lfunc_begin16:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movq	%rsi, %rdx
	movslq	%edi, %rsi
	leaq	__ZN8lifetime4main17h6ed9b53b0ea9bc36E(%rip), %rdi
	callq	__ZN3std2rt10lang_start17h172c3abe4c50711bE
	popq	%rbp
	retq
Lfunc_end16:
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3
l___unnamed_1:
	.quad	__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0a406dacefb1fee8E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h5d55519c156b3c13E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3f611c7bc2abae17E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3f611c7bc2abae17E

	.section	__TEXT,__const
__ZN8lifetime5THING17h57d16ec768adf029E:
	.byte	0

l___unnamed_2:
	.byte	97

	.section	__DATA,__const
	.p2align	3
__ZN8lifetime4main2X217hd5b4d4d907fc2bfbE:
	.quad	l___unnamed_2
	.asciz	"\001\000\000\000\000\000\000"

	.section	__DWARF,__debug_abbrev,regular,debug
Lsection_abbrev:
	.byte	1
	.byte	17
	.byte	1
	.byte	37
	.byte	14
	.byte	19
	.byte	5
	.byte	3
	.byte	14
	.byte	16
	.byte	6
	.byte	27
	.byte	14
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	0
	.byte	0
	.byte	2
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	2
	.byte	10
	.byte	0
	.byte	0
	.byte	3
	.byte	19
	.byte	0
	.byte	29
	.byte	19
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	4
	.byte	57
	.byte	1
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	5
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	6
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	10
	.byte	0
	.byte	0
	.byte	7
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	64
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	8
	.byte	52
	.byte	0
	.byte	2
	.byte	10
	.byte	3
	.byte	14
	.ascii	"\210\001"
	.byte	15
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	9
	.byte	47
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	10
	.byte	5
	.byte	0
	.byte	2
	.byte	10
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	11
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	64
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	0
	.byte	0
	.byte	12
	.byte	11
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	0
	.byte	0
	.byte	13
	.byte	52
	.byte	0
	.byte	2
	.byte	10
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	14
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	64
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	15
	.byte	5
	.byte	0
	.byte	2
	.byte	10
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	16
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	51
	.byte	6
	.byte	0
	.byte	0
	.byte	17
	.byte	21
	.byte	0
	.byte	0
	.byte	0
	.byte	18
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	2
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	0
	.byte	0
	.byte	19
	.byte	19
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	20
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	2
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	0
	.byte	0
	.byte	21
	.byte	46
	.byte	0
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	64
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	22
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	64
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	106
	.byte	12
	.byte	0
	.byte	0
	.byte	23
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	62
	.byte	11
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	24
	.byte	5
	.byte	0
	.byte	2
	.byte	10
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	25
	.byte	51
	.byte	1
	.byte	0
	.byte	0
	.byte	26
	.byte	25
	.byte	1
	.byte	0
	.byte	0
	.byte	27
	.byte	52
	.byte	0
	.byte	2
	.byte	10
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	28
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	1
	.byte	64
	.byte	10
	.ascii	"\207@"
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.ascii	"\207\001"
	.byte	12
	.byte	0
	.byte	0
	.byte	0
	.section	__DWARF,__debug_info,regular,debug
Lsection_info:
Lcu_begin0:
.set Lset0, Ldebug_info_end0-Ldebug_info_start0
	.long	Lset0
Ldebug_info_start0:
	.short	2
.set Lset1, Lsection_abbrev-Lsection_abbrev
	.long	Lset1
	.byte	8
	.byte	1
	.long	0
	.short	28
	.long	57
.set Lset2, Lline_table_start0-Lsection_line
	.long	Lset2
	.long	88
	.quad	Lfunc_begin0
	.quad	Lfunc_end15
	.byte	2
	.long	149
	.long	65
	.byte	9
	.byte	3
	.quad	l___unnamed_1
	.byte	3
	.long	91
	.long	149
	.byte	0
	.byte	8
	.byte	4
	.long	156
	.byte	4
	.long	160
	.byte	4
	.long	163
	.byte	5
	.long	174
	.byte	8
	.byte	8
	.byte	6
	.long	186
	.long	599
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	7
	.quad	Lfunc_begin2
	.quad	Lfunc_end2
	.byte	1
	.byte	86
	.long	550
	.long	534
	.byte	3
	.byte	63
	.long	1690
	.byte	8
	.byte	3
	.byte	145
	.byte	120
	.byte	6
	.long	186
	.byte	1
	.byte	3
	.byte	58
	.long	599
	.byte	9
	.long	1683
	.long	1322
	.byte	0
	.byte	0
	.byte	7
	.quad	Lfunc_begin1
	.quad	Lfunc_end1
	.byte	1
	.byte	86
	.long	491
	.long	476
	.byte	3
	.byte	57
	.long	1669
	.byte	10
	.byte	2
	.byte	145
	.byte	104
	.long	186
	.byte	3
	.byte	58
	.long	599
	.byte	10
	.byte	2
	.byte	145
	.byte	112
	.long	1955
	.byte	3
	.byte	59
	.long	1669
	.byte	10
	.byte	2
	.byte	145
	.byte	120
	.long	1960
	.byte	3
	.byte	60
	.long	1710
	.byte	9
	.long	1683
	.long	1322
	.byte	0
	.byte	0
	.byte	4
	.long	336
	.byte	4
	.long	347
	.byte	11
	.quad	Lfunc_begin0
	.quad	Lfunc_end0
	.byte	1
	.byte	86
	.long	396
	.long	357
	.byte	2
	.byte	121
	.byte	10
	.byte	2
	.byte	145
	.byte	104
	.long	1953
	.byte	2
	.byte	121
	.long	599
	.byte	12
	.quad	Ltmp4
	.quad	Ltmp5
	.byte	13
	.byte	2
	.byte	145
	.byte	96
	.long	1289
	.byte	2
	.byte	125
	.long	1683
	.byte	0
	.byte	9
	.long	599
	.long	1929
	.byte	9
	.long	1683
	.long	1322
	.byte	0
	.byte	0
	.byte	0
	.byte	4
	.long	623
	.byte	4
	.long	627
	.byte	4
	.long	632
	.byte	4
	.long	640
	.byte	5
	.long	655
	.byte	1
	.byte	1
	.byte	6
	.long	664
	.long	995
	.byte	1
	.byte	2
	.byte	35
	.byte	0
	.byte	14
	.quad	Lfunc_begin3
	.quad	Lfunc_end3
	.byte	1
	.byte	86
	.long	675
	.long	668
	.byte	4
	.short	468
	.long	1690
	.byte	15
	.byte	2
	.byte	145
	.byte	120
	.long	1982
	.byte	4
	.short	468
	.long	1723
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	4
	.long	632
	.byte	4
	.long	1618
	.byte	14
	.quad	Lfunc_begin12
	.quad	Lfunc_end12
	.byte	1
	.byte	86
	.long	1635
	.long	1628
	.byte	10
	.short	2023
	.long	1690
	.byte	15
	.byte	2
	.byte	145
	.byte	120
	.long	1982
	.byte	10
	.short	2023
	.long	1683
	.byte	0
	.byte	0
	.byte	4
	.long	1722
	.byte	14
	.quad	Lfunc_begin13
	.quad	Lfunc_end13
	.byte	1
	.byte	86
	.long	1732
	.long	1628
	.byte	10
	.short	2057
	.long	1690
	.byte	15
	.byte	2
	.byte	145
	.byte	127
	.long	1982
	.byte	10
	.short	2057
	.long	576
	.byte	0
	.byte	0
	.byte	5
	.long	655
	.byte	1
	.byte	1
	.byte	6
	.long	664
	.long	384
	.byte	1
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.long	612
	.long	191
	.long	0
	.byte	17
	.byte	4
	.long	196
	.byte	18
	.long	205
	.long	643
	.byte	1
	.byte	4
	.byte	9
	.byte	3
	.quad	__ZN8lifetime5THING17h57d16ec768adf029E
	.long	213
	.byte	19
	.long	211
	.byte	0
	.byte	1
	.byte	4
	.long	186
	.byte	20
	.long	252
	.long	948
	.byte	1
	.byte	31
	.byte	1
	.byte	9
	.byte	3
	.quad	__ZN8lifetime4main2X217hd5b4d4d907fc2bfbE
	.long	295
	.byte	0
	.byte	21
	.quad	Lfunc_begin14
	.quad	Lfunc_end14
	.byte	1
	.byte	86
	.long	1845
	.long	1833
	.byte	1
	.byte	5
	.long	1697
	.byte	22
	.quad	Lfunc_begin15
	.quad	Lfunc_end15
	.byte	1
	.byte	86
	.long	1891
	.long	186
	.byte	1
	.byte	9
	.byte	1
	.byte	12
	.quad	Ltmp40
	.quad	Ltmp42
	.byte	8
	.byte	3
	.byte	145
	.ascii	"\200\177"
	.long	2024
	.byte	1
	.byte	1
	.byte	20
	.long	948
	.byte	12
	.quad	Ltmp41
	.quad	Ltmp42
	.byte	8
	.byte	3
	.byte	145
	.ascii	"\220\177"
	.long	2027
	.byte	1
	.byte	1
	.byte	21
	.long	948
	.byte	0
	.byte	0
	.byte	12
	.quad	Ltmp43
	.quad	Ltmp46
	.byte	8
	.byte	3
	.byte	145
	.ascii	"\260\177"
	.long	2030
	.byte	1
	.byte	1
	.byte	30
	.long	948
	.byte	12
	.quad	Ltmp44
	.quad	Ltmp46
	.byte	8
	.byte	2
	.byte	145
	.byte	64
	.long	2024
	.byte	1
	.byte	1
	.byte	36
	.long	948
	.byte	12
	.quad	Ltmp45
	.quad	Ltmp46
	.byte	8
	.byte	2
	.byte	145
	.byte	80
	.long	2027
	.byte	1
	.byte	1
	.byte	37
	.long	948
	.byte	0
	.byte	0
	.byte	0
	.byte	12
	.quad	Ltmp47
	.quad	Ltmp48
	.byte	8
	.byte	2
	.byte	145
	.byte	112
	.long	2024
	.byte	1
	.byte	1
	.byte	46
	.long	1697
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	255
	.byte	16
	.byte	8
	.byte	6
	.long	260
	.long	982
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	6
	.long	282
	.long	1002
	.byte	8
	.byte	2
	.byte	35
	.byte	8
	.byte	0
	.byte	16
	.long	995
	.long	269
	.long	0
	.byte	23
	.long	279
	.byte	7
	.byte	1
	.byte	23
	.long	289
	.byte	7
	.byte	8
	.byte	4
	.long	752
	.byte	4
	.long	757
	.byte	4
	.long	761
	.byte	4
	.long	770
	.byte	7
	.quad	Lfunc_begin4
	.quad	Lfunc_end4
	.byte	1
	.byte	86
	.long	825
	.long	777
	.byte	5
	.byte	227
	.long	1690
	.byte	24
	.byte	2
	.byte	145
	.byte	120
	.byte	5
	.byte	227
	.long	1736
	.byte	24
	.byte	2
	.byte	145
	.byte	112
	.byte	5
	.byte	227
	.long	1683
	.byte	9
	.long	91
	.long	1938
	.byte	9
	.long	1683
	.long	1943
	.byte	0
	.byte	7
	.quad	Lfunc_begin5
	.quad	Lfunc_end5
	.byte	1
	.byte	86
	.long	916
	.long	777
	.byte	5
	.byte	227
	.long	1690
	.byte	24
	.byte	2
	.byte	145
	.byte	96
	.byte	5
	.byte	227
	.long	91
	.byte	24
	.byte	2
	.byte	145
	.byte	104
	.byte	5
	.byte	227
	.long	1683
	.byte	9
	.long	91
	.long	1938
	.byte	9
	.long	1683
	.long	1943
	.byte	0
	.byte	11
	.quad	Lfunc_begin6
	.quad	Lfunc_end6
	.byte	1
	.byte	86
	.long	995
	.long	975
	.byte	5
	.byte	227
	.byte	24
	.byte	2
	.byte	145
	.byte	120
	.byte	5
	.byte	227
	.long	599
	.byte	24
	.byte	2
	.byte	145
	.byte	112
	.byte	5
	.byte	227
	.long	1683
	.byte	9
	.long	599
	.long	1938
	.byte	9
	.long	1683
	.long	1943
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	4
	.long	1054
	.byte	11
	.quad	Lfunc_begin7
	.quad	Lfunc_end7
	.byte	1
	.byte	86
	.long	1106
	.long	1058
	.byte	6
	.byte	188
	.byte	24
	.byte	2
	.byte	145
	.byte	120
	.byte	6
	.byte	188
	.long	1736
	.byte	9
	.long	91
	.long	1322
	.byte	0
	.byte	0
	.byte	4
	.long	1226
	.byte	11
	.quad	Lfunc_begin8
	.quad	Lfunc_end8
	.byte	1
	.byte	86
	.long	1245
	.long	1231
	.byte	7
	.byte	159
	.byte	10
	.byte	2
	.byte	145
	.byte	120
	.long	2014
	.byte	7
	.byte	159
	.long	1683
	.byte	9
	.long	1683
	.long	1322
	.byte	0
	.byte	0
	.byte	4
	.long	1289
	.byte	5
	.long	1296
	.byte	8
	.byte	8
	.byte	25
	.byte	26
	.byte	6
	.long	1313
	.long	1387
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	1313
	.byte	8
	.byte	8
	.byte	9
	.long	1669
	.long	1322
	.byte	9
	.long	1676
	.long	1326
	.byte	6
	.long	664
	.long	1669
	.byte	8
	.byte	2
	.byte	35
	.byte	0
	.byte	0
	.byte	14
	.quad	Lfunc_begin9
	.quad	Lfunc_end9
	.byte	1
	.byte	86
	.long	1346
	.long	1328
	.byte	8
	.short	1424
	.long	1669
	.byte	15
	.byte	2
	.byte	145
	.byte	104
	.long	1982
	.byte	8
	.short	1424
	.long	1363
	.byte	12
	.quad	Ltmp27
	.quad	Ltmp28
	.byte	27
	.byte	2
	.byte	145
	.byte	112
	.long	2020
	.byte	8
	.short	1427
	.long	1676
	.byte	0
	.byte	9
	.long	1669
	.long	1322
	.byte	9
	.long	1676
	.long	1326
	.byte	0
	.byte	0
	.byte	0
	.byte	4
	.long	1411
	.byte	4
	.long	1419
	.byte	28
	.quad	Lfunc_begin10
	.quad	Lfunc_end10
	.byte	1
	.byte	86
	.long	1436
	.long	1428
	.byte	9
	.short	547
	.byte	1
	.byte	15
	.byte	2
	.byte	145
	.byte	120
	.long	2022
	.byte	9
	.short	547
	.long	1676
	.byte	9
	.long	1676
	.long	1322
	.byte	0
	.byte	0
	.byte	4
	.long	1517
	.byte	28
	.quad	Lfunc_begin11
	.quad	Lfunc_end11
	.byte	1
	.byte	86
	.long	1537
	.long	1526
	.byte	9
	.short	539
	.byte	1
	.byte	15
	.byte	2
	.byte	145
	.byte	120
	.long	1982
	.byte	9
	.short	539
	.long	1676
	.byte	9
	.long	1676
	.long	1322
	.byte	9
	.long	1676
	.long	1948
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	23
	.long	1316
	.byte	5
	.byte	8
	.byte	23
	.long	1324
	.byte	7
	.byte	0
	.byte	23
	.long	1931
	.byte	7
	.byte	0
	.byte	23
	.long	1934
	.byte	5
	.byte	4
	.byte	16
	.long	643
	.long	1950
	.long	0
	.byte	16
	.long	982
	.long	1965
	.long	0
	.byte	16
	.long	384
	.long	1987
	.long	0
	.byte	16
	.long	91
	.long	1997
	.long	0
	.byte	0
Ldebug_info_end0:
	.section	__DATA,__const
Lsec_end0:
	.section	__TEXT,__const
Lsec_end1:
	.section	__TEXT,__text,regular,pure_instructions
Lsec_end2:
	.section	__DWARF,__debug_aranges,regular,debug
	.long	76
	.short	2
.set Lset3, Lcu_begin0-Lsection_info
	.long	Lset3
	.byte	8
	.byte	0
	.space	4,255
	.quad	l___unnamed_1
.set Lset4, Lsec_end0-l___unnamed_1
	.quad	Lset4
	.quad	__ZN8lifetime5THING17h57d16ec768adf029E
.set Lset5, Lsec_end1-__ZN8lifetime5THING17h57d16ec768adf029E
	.quad	Lset5
	.quad	Lfunc_begin0
.set Lset6, Lsec_end2-Lfunc_begin0
	.quad	Lset6
	.quad	0
	.quad	0
	.section	__DWARF,__debug_str,regular,debug
Linfo_string:
	.asciz	"clang LLVM (rustc version 1.56.1 (59eed8a2a 2021-11-01))"
	.asciz	"src/main.rs/@/3sjfgp6bfo8qoo3c"
	.asciz	"/Volumes/data/4_github/learn-rust/100-rust-snippets/lifetime"
	.asciz	"vtable"
	.asciz	"std"
	.asciz	"rt"
	.asciz	"lang_start"
	.asciz	"{closure#0}"
	.asciz	"main"
	.asciz	"fn()"
	.asciz	"lifetime"
	.asciz	"THING"
	.asciz	"A"
	.asciz	"_ZN8lifetime5THING17h57d16ec768adf029E"
	.asciz	"X2"
	.asciz	"&str"
	.asciz	"data_ptr"
	.asciz	"*const u8"
	.asciz	"u8"
	.asciz	"length"
	.asciz	"usize"
	.asciz	"_ZN8lifetime4main2X217hd5b4d4d907fc2bfbE"
	.asciz	"sys_common"
	.asciz	"backtrace"
	.asciz	"__rust_begin_short_backtrace<fn(), ()>"
	.asciz	"_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hcfe3b45c960fa275E"
	.asciz	"lang_start<()>"
	.asciz	"_ZN3std2rt10lang_start17h172c3abe4c50711bE"
	.asciz	"{closure#0}<()>"
	.asciz	"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3f611c7bc2abae17E"
	.asciz	"sys"
	.asciz	"unix"
	.asciz	"process"
	.asciz	"process_common"
	.asciz	"ExitCode"
	.asciz	"__0"
	.asciz	"as_i32"
	.asciz	"_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217h5c510ae19c3bfc1aE"
	.asciz	"core"
	.asciz	"ops"
	.asciz	"function"
	.asciz	"FnOnce"
	.asciz	"call_once<std::rt::lang_start::{closure#0}, ()>"
	.asciz	"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h5d55519c156b3c13E"
	.asciz	"_ZN4core3ops8function6FnOnce9call_once17h0a4b30c31cd99dbfE"
	.asciz	"call_once<fn(), ()>"
	.asciz	"_ZN4core3ops8function6FnOnce9call_once17h3e982e1eb2657badE"
	.asciz	"ptr"
	.asciz	"drop_in_place<std::rt::lang_start::{closure#0}>"
	.asciz	"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0a406dacefb1fee8E"
	.asciz	"hint"
	.asciz	"black_box<()>"
	.asciz	"_ZN4core4hint9black_box17h7866d660ef3e8471E"
	.asciz	"result"
	.asciz	"Result<isize, !>"
	.asciz	"Ok"
	.asciz	"isize"
	.asciz	"T"
	.asciz	"!"
	.asciz	"E"
	.asciz	"into_ok<isize, !>"
	.asciz	"_ZN4core6result19Result$LT$T$C$E$GT$7into_ok17h71d4d9dd8f81654fE"
	.asciz	"convert"
	.asciz	"{impl#4}"
	.asciz	"from<!>"
	.asciz	"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h4a35e1e720d56b42E"
	.asciz	"{impl#3}"
	.asciz	"into<!, !>"
	.asciz	"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h3492721134b3c734E"
	.asciz	"{impl#48}"
	.asciz	"report"
	.asciz	"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h4e8c5194f038480cE"
	.asciz	"{impl#52}"
	.asciz	"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17hf74eeb29a67faa5bE"
	.asciz	"getInstance"
	.asciz	"_ZN8lifetime11getInstance17hcb8115af624794f7E"
	.asciz	"_ZN8lifetime4main17h6ed9b53b0ea9bc36E"
	.asciz	"F"
	.asciz	"()"
	.asciz	"i32"
	.asciz	"Self"
	.asciz	"Args"
	.asciz	"U"
	.asciz	"&A"
	.asciz	"f"
	.asciz	"argc"
	.asciz	"argv"
	.asciz	"*const *const u8"
	.asciz	"self"
	.asciz	"&ExitCode"
	.asciz	"*mut {closure#0}"
	.asciz	"dummy"
	.asciz	"e"
	.asciz	"t"
	.asciz	"_a"
	.asciz	"_b"
	.asciz	"X1"
	.section	__DWARF,__apple_names,regular,debug
Lnames_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	17
	.long	35
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	0
	.long	1
	.long	-1
	.long	4
	.long	5
	.long	7
	.long	8
	.long	9
	.long	11
	.long	16
	.long	19
	.long	21
	.long	24
	.long	27
	.long	29
	.long	32
	.long	33
	.long	-300363073
	.long	1108834011
	.long	2090499946
	.long	-338731392
	.long	236439199
	.long	706936742
	.long	2067556154
	.long	5862863
	.long	-399237753
	.long	690877610
	.long	-1049275660
	.long	178415705
	.long	2040266925
	.long	2104400853
	.long	-2115157928
	.long	-1554419150
	.long	1119701762
	.long	-2120925636
	.long	-109322079
	.long	422451489
	.long	568789665
	.long	46180188
	.long	-1299778816
	.long	-26309445
	.long	359682901
	.long	1995179767
	.long	-607947393
	.long	1078341293
	.long	-1584674113
	.long	596228451
	.long	-1682213533
	.long	-1121138342
	.long	1702852598
	.long	1845844053
	.long	-226866906
.set Lset7, LNames32-Lnames_begin
	.long	Lset7
.set Lset8, LNames26-Lnames_begin
	.long	Lset8
.set Lset9, LNames20-Lnames_begin
	.long	Lset9
.set Lset10, LNames12-Lnames_begin
	.long	Lset10
.set Lset11, LNames0-Lnames_begin
	.long	Lset11
.set Lset12, LNames18-Lnames_begin
	.long	Lset12
.set Lset13, LNames10-Lnames_begin
	.long	Lset13
.set Lset14, LNames3-Lnames_begin
	.long	Lset14
.set Lset15, LNames1-Lnames_begin
	.long	Lset15
.set Lset16, LNames19-Lnames_begin
	.long	Lset16
.set Lset17, LNames5-Lnames_begin
	.long	Lset17
.set Lset18, LNames8-Lnames_begin
	.long	Lset18
.set Lset19, LNames21-Lnames_begin
	.long	Lset19
.set Lset20, LNames22-Lnames_begin
	.long	Lset20
.set Lset21, LNames25-Lnames_begin
	.long	Lset21
.set Lset22, LNames4-Lnames_begin
	.long	Lset22
.set Lset23, LNames16-Lnames_begin
	.long	Lset23
.set Lset24, LNames27-Lnames_begin
	.long	Lset24
.set Lset25, LNames13-Lnames_begin
	.long	Lset25
.set Lset26, LNames14-Lnames_begin
	.long	Lset26
.set Lset27, LNames34-Lnames_begin
	.long	Lset27
.set Lset28, LNames11-Lnames_begin
	.long	Lset28
.set Lset29, LNames31-Lnames_begin
	.long	Lset29
.set Lset30, LNames28-Lnames_begin
	.long	Lset30
.set Lset31, LNames6-Lnames_begin
	.long	Lset31
.set Lset32, LNames7-Lnames_begin
	.long	Lset32
.set Lset33, LNames30-Lnames_begin
	.long	Lset33
.set Lset34, LNames2-Lnames_begin
	.long	Lset34
.set Lset35, LNames29-Lnames_begin
	.long	Lset35
.set Lset36, LNames15-Lnames_begin
	.long	Lset36
.set Lset37, LNames33-Lnames_begin
	.long	Lset37
.set Lset38, LNames9-Lnames_begin
	.long	Lset38
.set Lset39, LNames24-Lnames_begin
	.long	Lset39
.set Lset40, LNames23-Lnames_begin
	.long	Lset40
.set Lset41, LNames17-Lnames_begin
	.long	Lset41
LNames32:
	.long	476
	.long	1
	.long	172
	.long	0
LNames26:
	.long	396
	.long	1
	.long	268
	.long	0
LNames20:
	.long	186
	.long	1
	.long	715
	.long	0
LNames12:
	.long	777
	.long	2
	.long	1029
	.long	1101
	.long	0
LNames0:
	.long	205
	.long	1
	.long	618
	.long	0
LNames18:
	.long	1346
	.long	1
	.long	1426
	.long	0
LNames10:
	.long	1833
	.long	1
	.long	682
	.long	0
LNames3:
	.long	252
	.long	1
	.long	655
	.long	0
LNames1:
	.long	1526
	.long	1
	.long	1601
	.long	0
LNames19:
	.long	825
	.long	1
	.long	1029
	.long	0
LNames5:
	.long	1428
	.long	1
	.long	1539
	.long	0
LNames8:
	.long	1635
	.long	1
	.long	469
	.long	0
LNames21:
	.long	295
	.long	1
	.long	655
	.long	0
LNames22:
	.long	1245
	.long	1
	.long	1304
	.long	0
LNames25:
	.long	534
	.long	1
	.long	112
	.long	0
LNames4:
	.long	1058
	.long	1
	.long	1249
	.long	0
LNames16:
	.long	1891
	.long	1
	.long	715
	.long	0
LNames27:
	.long	550
	.long	1
	.long	112
	.long	0
LNames13:
	.long	213
	.long	1
	.long	618
	.long	0
LNames14:
	.long	1628
	.long	2
	.long	469
	.long	525
	.long	0
LNames34:
	.long	975
	.long	1
	.long	1173
	.long	0
LNames11:
	.long	995
	.long	1
	.long	1173
	.long	0
LNames31:
	.long	675
	.long	1
	.long	404
	.long	0
LNames28:
	.long	1436
	.long	1
	.long	1539
	.long	0
LNames6:
	.long	1732
	.long	1
	.long	525
	.long	0
LNames7:
	.long	1845
	.long	1
	.long	682
	.long	0
LNames30:
	.long	916
	.long	1
	.long	1101
	.long	0
LNames2:
	.long	1106
	.long	1
	.long	1249
	.long	0
LNames29:
	.long	357
	.long	1
	.long	268
	.long	0
LNames15:
	.long	149
	.long	1
	.long	46
	.long	0
LNames33:
	.long	1328
	.long	1
	.long	1426
	.long	0
LNames9:
	.long	1537
	.long	1
	.long	1601
	.long	0
LNames24:
	.long	491
	.long	1
	.long	172
	.long	0
LNames23:
	.long	1231
	.long	1
	.long	1304
	.long	0
LNames17:
	.long	668
	.long	1
	.long	404
	.long	0
	.section	__DWARF,__apple_objc,regular,debug
Lobjc_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	1
	.long	0
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	-1
	.section	__DWARF,__apple_namespac,regular,debug
Lnamespac_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	11
	.long	23
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	-1
	.long	0
	.long	1
	.long	3
	.long	5
	.long	6
	.long	11
	.long	13
	.long	14
	.long	18
	.long	21
	.long	193506160
	.long	2090329144
	.long	-712886363
	.long	-1229807316
	.long	-746933562
	.long	193502907
	.long	193501687
	.long	193506340
	.long	-1536477282
	.long	-1536476391
	.long	-1430835988
	.long	5863787
	.long	2090801609
	.long	2090499946
	.long	-1738516699
	.long	-1738516666
	.long	-1351661516
	.long	-1019809820
	.long	422565636
	.long	2090156110
	.long	-1290020034
	.long	1883124308
	.long	-735823797
.set Lset42, Lnamespac10-Lnamespac_begin
	.long	Lset42
.set Lset43, Lnamespac16-Lnamespac_begin
	.long	Lset43
.set Lset44, Lnamespac0-Lnamespac_begin
	.long	Lset44
.set Lset45, Lnamespac7-Lnamespac_begin
	.long	Lset45
.set Lset46, Lnamespac1-Lnamespac_begin
	.long	Lset46
.set Lset47, Lnamespac18-Lnamespac_begin
	.long	Lset47
.set Lset48, Lnamespac15-Lnamespac_begin
	.long	Lset48
.set Lset49, Lnamespac22-Lnamespac_begin
	.long	Lset49
.set Lset50, Lnamespac20-Lnamespac_begin
	.long	Lset50
.set Lset51, Lnamespac17-Lnamespac_begin
	.long	Lset51
.set Lset52, Lnamespac8-Lnamespac_begin
	.long	Lset52
.set Lset53, Lnamespac6-Lnamespac_begin
	.long	Lset53
.set Lset54, Lnamespac4-Lnamespac_begin
	.long	Lset54
.set Lset55, Lnamespac5-Lnamespac_begin
	.long	Lset55
.set Lset56, Lnamespac3-Lnamespac_begin
	.long	Lset56
.set Lset57, Lnamespac21-Lnamespac_begin
	.long	Lset57
.set Lset58, Lnamespac13-Lnamespac_begin
	.long	Lset58
.set Lset59, Lnamespac2-Lnamespac_begin
	.long	Lset59
.set Lset60, Lnamespac12-Lnamespac_begin
	.long	Lset60
.set Lset61, Lnamespac9-Lnamespac_begin
	.long	Lset61
.set Lset62, Lnamespac19-Lnamespac_begin
	.long	Lset62
.set Lset63, Lnamespac14-Lnamespac_begin
	.long	Lset63
.set Lset64, Lnamespac11-Lnamespac_begin
	.long	Lset64
Lnamespac10:
	.long	156
	.long	1
	.long	76
	.long	0
Lnamespac16:
	.long	1226
	.long	1
	.long	1299
	.long	0
Lnamespac0:
	.long	347
	.long	1
	.long	263
	.long	0
Lnamespac7:
	.long	336
	.long	1
	.long	258
	.long	0
Lnamespac1:
	.long	1411
	.long	1
	.long	1529
	.long	0
Lnamespac18:
	.long	1054
	.long	1
	.long	1244
	.long	0
Lnamespac15:
	.long	757
	.long	1
	.long	1014
	.long	0
Lnamespac22:
	.long	623
	.long	1
	.long	364
	.long	0
Lnamespac20:
	.long	1618
	.long	1
	.long	464
	.long	0
Lnamespac17:
	.long	1722
	.long	1
	.long	520
	.long	0
Lnamespac8:
	.long	640
	.long	1
	.long	379
	.long	0
Lnamespac6:
	.long	160
	.long	1
	.long	81
	.long	0
Lnamespac4:
	.long	627
	.long	1
	.long	369
	.long	0
Lnamespac5:
	.long	186
	.long	1
	.long	650
	.long	0
Lnamespac3:
	.long	1517
	.long	1
	.long	1596
	.long	0
Lnamespac21:
	.long	1419
	.long	1
	.long	1534
	.long	0
Lnamespac13:
	.long	196
	.long	1
	.long	613
	.long	0
Lnamespac2:
	.long	632
	.long	2
	.long	374
	.long	459
	.long	0
Lnamespac12:
	.long	1289
	.long	1
	.long	1358
	.long	0
Lnamespac9:
	.long	752
	.long	1
	.long	1009
	.long	0
Lnamespac19:
	.long	770
	.long	1
	.long	1024
	.long	0
Lnamespac14:
	.long	163
	.long	1
	.long	86
	.long	0
Lnamespac11:
	.long	761
	.long	1
	.long	1019
	.long	0
	.section	__DWARF,__apple_types,regular,debug
Ltypes_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	9
	.long	19
	.long	20
	.long	0
	.long	3
	.short	1
	.short	6
	.short	3
	.short	5
	.short	4
	.short	11
	.long	0
	.long	1
	.long	5
	.long	7
	.long	-1
	.long	9
	.long	14
	.long	16
	.long	-1
	.long	177606
	.long	1410053581
	.long	-776881299
	.long	-713725437
	.long	-51932352
	.long	5861270
	.long	5863826
	.long	1006996602
	.long	-1669181905
	.long	177638
	.long	5861228
	.long	5862623
	.long	193493075
	.long	2087968388
	.long	596228451
	.long	2127712596
	.long	262925161
	.long	277156213
	.long	2090260330
.set Lset65, Ltypes2-Ltypes_begin
	.long	Lset65
.set Lset66, Ltypes7-Ltypes_begin
	.long	Lset66
.set Lset67, Ltypes6-Ltypes_begin
	.long	Lset67
.set Lset68, Ltypes17-Ltypes_begin
	.long	Lset68
.set Lset69, Ltypes16-Ltypes_begin
	.long	Lset69
.set Lset70, Ltypes13-Ltypes_begin
	.long	Lset70
.set Lset71, Ltypes9-Ltypes_begin
	.long	Lset71
.set Lset72, Ltypes14-Ltypes_begin
	.long	Lset72
.set Lset73, Ltypes8-Ltypes_begin
	.long	Lset73
.set Lset74, Ltypes1-Ltypes_begin
	.long	Lset74
.set Lset75, Ltypes5-Ltypes_begin
	.long	Lset75
.set Lset76, Ltypes15-Ltypes_begin
	.long	Lset76
.set Lset77, Ltypes10-Ltypes_begin
	.long	Lset77
.set Lset78, Ltypes0-Ltypes_begin
	.long	Lset78
.set Lset79, Ltypes18-Ltypes_begin
	.long	Lset79
.set Lset80, Ltypes11-Ltypes_begin
	.long	Lset80
.set Lset81, Ltypes3-Ltypes_begin
	.long	Lset81
.set Lset82, Ltypes12-Ltypes_begin
	.long	Lset82
.set Lset83, Ltypes4-Ltypes_begin
	.long	Lset83
Ltypes2:
	.long	1324
	.long	1
	.long	1676
	.short	36
	.byte	0
	.long	0
Ltypes7:
	.long	1997
	.long	1
	.long	1736
	.short	15
	.byte	0
	.long	0
Ltypes6:
	.long	174
	.long	1
	.long	91
	.short	19
	.byte	0
	.long	0
Ltypes17:
	.long	269
	.long	1
	.long	982
	.short	15
	.byte	0
	.long	0
Ltypes16:
	.long	1987
	.long	1
	.long	1723
	.short	15
	.byte	0
	.long	0
Ltypes13:
	.long	1931
	.long	1
	.long	1683
	.short	36
	.byte	0
	.long	0
Ltypes9:
	.long	279
	.long	1
	.long	995
	.short	36
	.byte	0
	.long	0
Ltypes14:
	.long	655
	.long	2
	.long	384
	.short	19
	.byte	0
	.long	576
	.short	19
	.byte	0
	.long	0
Ltypes8:
	.long	1296
	.long	1
	.long	1363
	.short	19
	.byte	0
	.long	0
Ltypes1:
	.long	211
	.long	1
	.long	643
	.short	19
	.byte	0
	.long	0
Ltypes5:
	.long	1950
	.long	1
	.long	1697
	.short	15
	.byte	0
	.long	0
Ltypes15:
	.long	1313
	.long	1
	.long	1387
	.short	19
	.byte	0
	.long	0
Ltypes10:
	.long	1934
	.long	1
	.long	1690
	.short	36
	.byte	0
	.long	0
Ltypes0:
	.long	255
	.long	1
	.long	948
	.short	19
	.byte	0
	.long	0
Ltypes18:
	.long	149
	.long	1
	.long	65
	.short	19
	.byte	0
	.long	0
Ltypes11:
	.long	1965
	.long	1
	.long	1710
	.short	15
	.byte	0
	.long	0
Ltypes3:
	.long	1316
	.long	1
	.long	1669
	.short	36
	.byte	0
	.long	0
Ltypes12:
	.long	289
	.long	1
	.long	1002
	.short	36
	.byte	0
	.long	0
Ltypes4:
	.long	191
	.long	1
	.long	599
	.short	15
	.byte	0
	.long	0
.subsections_via_symbols
	.section	__DWARF,__debug_line,regular,debug
Lsection_line:
Lline_table_start0:
