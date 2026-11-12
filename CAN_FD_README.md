
## 做了什么

- `src/board.rs`：CAN1 仍使用 PD0（RX）和 PD1（TX），仲裁段保持 1 Mbit/s；启用 CAN FD 和 BRS，数据段配置为 2 Mbit/s。
- `src/protocol.rs`：新增 `CANFD TX` 和 `CANFD RX` 控制命令。发送命令目前要求恰好 12 字节，接收命令报告标准帧 ID、数据和是否使用 BRS。
- `host.py`：新增 `canfd-send`、`canfd-recv` 子命令，并检查发送数据为有效的 12 字节十六进制内容。

原有的 Classical CAN 命令和帧支持仍保留。CAN FD 与 Classical CAN 共用 CAN1 的引脚和板载收发器，不需要新增 GPIO 配置。

## 基本原理

CAN FD 扩展了 Classical CAN 的数据长度，单帧最多可携带 64 字节。使用 BRS（Bit Rate Switching）时，帧在仲裁阶段使用 nominal 位率，进入数据阶段后切换到 data 位率，并在帧尾切回。

本配置使用 1 Mbit/s 仲裁位率和 2 Mbit/s 数据位率。通信两端必须支持 CAN FD，并匹配位率、时序和 BRS 设置；只支持 Classical CAN 的节点不能参与 FD 帧通信。

## 测试命令

在控制串口上发送 12 字节 CAN FD 标准帧：

```sh
python3 host.py CONTROL_PORT canfd-send 123 00112233445566778899AABB
python3 host.py CONTROL_PORT canfd-recv
```


