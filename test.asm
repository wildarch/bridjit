mov rax, rdi
push rax
mov rsi, rax
mov rax, 1
mov rdx, 1
mov rdi, 1
syscall
pop rax
