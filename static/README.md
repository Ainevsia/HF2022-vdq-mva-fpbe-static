# Misc | static (6 solves)

## Description

Just a simple contract

```
nc     47.107.31.31 42000  
faucet 47.107.31.31 48080  
geth   47.107.31.31 48545  
```

## Solution

题目给出了Solidity[源码](./to_player/challenge.sol)如下

```js
pragma solidity =0.8.12;

contract Challenge {
    uint constant fee = 100000;
    uint constant max_code_size = 0x80;

    event SendFlag(address);

    function solve() public {
        uint answer;
        bool success;
        bytes memory result;

        assembly {
            answer := extcodesize(caller())
        }
        require(answer < max_code_size);

        (success, result) = msg.sender.staticcall{gas:fee}("");
        answer = uint(bytes32(result));
        require(success && answer == 1);

        (success, result) = msg.sender.staticcall{gas:fee}("");
        answer = uint(bytes32(result));
        require(success && answer == 2);

        emit SendFlag(msg.sender);
    }
}

```

可见，合约在检查完`extcodesize(msg.sender) < 0x80`之后进行了两次`staticcall`，并要求两次对外部同一个地址的调用分别返回整数1和2，最后发送SendFlag事件。

比较值得注意的是两次`staticcall`都指定了相同的gas量：`10w`。

搜索[staticcall](https://eips.ethereum.org/EIPS/eip-214)可知其特性为：本次外部地址的调用不允许进行任何状态的改变，也就是说被调合约不能使用storage变量记录自身的状态。

那么该如何在两次所给gas相同、参数相同、不用storage变量的两次“完全一模一样”的staticcall中返回不同的值呢？

排除所有简单的“是否真的相同”的猜想之后可以察觉到上述“两次所给gas相同”的条件是不一定能够成立的。

被调合约实际gas余量不仅取决于staticcall时所给定的gas参数，还取决于调用合约自己本身的gas余量，换句话说，当Challenge合约自身在进行staticcall调用时的gas余量少于其为当前staticcall声明的gas量(10w)时，被调合约能够实际使用的gas量必然是达不到10w的。

本题的预期解法为根据gas的余量来分别判断两次staticcall，令Challenge合约第一次staticcall时能够以满gas进入，第二次staticcall时以非满gas的状态进入，进行状态的判断，进而返回不同的值。

由于同样的程序在不同的geth版本上消耗的gas量各不相同，题目还给出了geth的配置文件[genesis.json](./to_player/genesis.json)，便于选手本地调试时给出与题目服务器一致的结果。

当然，明确了解体思路是给一个特定的值的gas后，爆破也是可以的。

要求`extcodesize(msg.sender) < 0x80`，用solidity源码编译出来的是不可能满足这个要求的，需要选手自己手写EVM字节码，0x80不是一个很紧的限制，足以完成题目所要求的功能。

> 阅读选手赛后提交的WP后意识到0x80限制可以使用proxy合约绕过，详见[EIP-1167: Minimal Proxy Contract](https://eips.ethereum.org/EIPS/eip-1167)

手写字节码可以真的手写，也可以使用工具，如 [ethereum/py-evm](https://github.com/ethereum/py-evm)和[Ainevsia/evm-assembler](https://github.com/Ainevsia/evm-assembler)，省去一些手动计算jump地址的烦恼。

下面以出题人写的[字节码](https://github.com/Ainevsia/evm-assembler/blob/main/gas.txt)为例：

```asm
    gas
    push3 0x01869e
    calldatasize
    push1 0
    lt
    push __attack__
    jumpi
    eq
    push __1__
    jumpi
    push1 2
    push __return__
    jump
__1__:
    jumpdest
    push1 1
    push __return__
    jump
__return__:
    jumpdest
    push1 0
    mstore
    push1 0x20
    push1 0
    return

__attack__:
    jumpdest
    push1 0
    push4 0x890d6908
    dup2
    mstore
    dup1
    push1 4
    push1 28
    dup3
    push20  0x066036e1F2C49EC994b9D2797932fED48230Ce2f
    push3 0x019258
    call
    push1 1
    eq
    push __graceful__
    jumpi
    revert
__graceful__:
    jumpdest
    stop
```

attacker合约首先根据calldatasize判断是否是challenge调用自身，随后根据gas余量返回不同的值。

exp中可以替换目标合约和所给的gas，~~（便于爆破）~~

[full exp](./exp.py)

## Flag

`HFCTF{Awesome_Ethereum_hacker_master_of_gas_AxJ80V97JZBn0i1JCkaA597iTmPNUcADmrq62lhf}`

## Side Note

- Infrustructure code: Great Thanks to [chainflag/eth-challenge-base](https://github.com/chainflag/eth-challenge-base).
- [London Harkfork](https://ethereum.org/en/history/)引入的[eip-1559](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1559.md)增加了冷热gas的区别，从genesis.json可以看出题目服务器环境还在Istanbul，不涉及冷热gas
- 阅读选手赛后提交的WP后意识到21年[首届中国可信区块链安全攻防大赛](https://www.scba.org.cn/?list_9.html)上出现过类似赛题