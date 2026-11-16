
## 任务目标

扩展任务有三个可选方向，本实现选择 CAN FD 方向：仲裁段 1 Mbit/s、数据段 5 Mbit/s，启用 BRS 和 TDC，并支持 12、32、64 字节数据帧。CAN1 仍使用 PD0（RX）、PD1（TX）和板载收发器；不需要在 `wiring.rs` 增加 GPIO。

## 位时序和 TDC

- FDCAN 内核时钟：80 MHz；一个最小时间量子（mtq）为 12.5 ns。
- 仲裁段：1 Mbit/s，保留原有 nominal timing。
- 数据段：prescaler=1、TSEG1=11、TSEG2=4、SJW=2，总计 16 tq/bit，即 5 Mbit/s；采样点为 12/16=75%。
- TDC：DBTP.TDC=1，TDCO=13 mtq（162.5 ns），TDCF=0（不启用滤波窗口）。TDCO 是在硬件测得收发器延时 TDCV 上增加的采样偏移，并非直接填写收发器的总延时。配置参考板卡资料中的收发器延时约 275 ns；对应的次级采样点约为 TDCV+162.5 ns，具体 TDCV 由硬件测量。

`embassy-stm32 0.6` 的 `DataBitTiming.transceiver_delay_compensation` 字段目前没有被该 HAL 实际写入 DBTP/TDCR。因此 `src/board.rs` 在 CAN 启动配置后通过 PAC 进入配置模式，设置 TDC 位和 TDCR，再返回正常模式。启动时必须在有 CAN 总线活动前完成配置。

## 控制协议与测试

通过控制串口发送十六进制数据，发送长度仅接受 12、32 或 64 字节：

```sh
python3 host.py CONTROL_PORT canfd-send 123 00112233445566778899AABB
python3 host.py CONTROL_PORT canfd-send 123 <64个十六进制字符>
python3 host.py CONTROL_PORT canfd-send 123 <128个十六进制字符>
python3 host.py CONTROL_PORT canfd-recv
```

固件的 `CANFD RX` 回复包含帧 ID、数据长度和 BRS 状态；数据按不超过 24 字节的 `DATA <offset> <hex>` 行分段，避免单个 USB CDC 包超过 64 字节。`host.py canfd-recv` 检查偏移并重组数据后打印完整帧。

Classical CAN 的 `CAN TX` / `CAN RX` 命令保持不变。CAN FD 对端必须支持 FD+BRS，并配置相同的仲裁与数据位率。此处仅做编译和协议实现验证，尚未上板或实测总线；TDC 效果、实际信号质量和互通性仍需硬件验收。

