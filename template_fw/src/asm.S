/*
 Stephen Marz
 6 October 2019
*/

.option norvc

.section .text.init

.global _start
_start:
.option push
.option norelax
	la gp, _global_pointer
.option pop
	/* Reset satp */
	csrw satp, zero
 
	# Set all bytes in the BSS section to zero.
	la a0, _bss_start
	la a1, _bss_end
	bgeu a0, a1, 2f
1:
	sd zero, (a0)
	addi a0, a0, 8
	bltu a0, a1, 1b
2: 
	la sp, _stack_end

	li t0, 0b11 << 11 | (1 << 13)
	csrw mstatus, t0

	jal zero, _start_rust
3:
	wfi
	j 3b
