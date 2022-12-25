# 嵌入式工作清单
***
##  基础环境
#### 软件环境
1. [x] 树莓派4B刷入Ubuntu操作系统，配置wifi连接
2. [x] MacOsx上Rust对树莓派4B(arm64)的交叉编译环境
3. [x] Rust gpio、pwm、i2c 驱动选型 rppal
4. [x] arp-scan 局域网扫描工具
5. [x] ssh 远程连接工具
6. [x] scp 远程上传工具
#### 硬件环境
1. [x] 树莓派小车拼装（树莓派4B）
   1. [x] 寻迹红外
   2. [x] 超声波探头、舵机
   3. [x] 摄像头、舵机
   4. [x] 电机驱动
   5. [x] 红外测距
   6. [x] 红外接收器
   7. [x] 扩展板
   8. [x] 电池

## 程序实验

#### 程序驱动
1. [x] GPIO 点亮LED
2. [x] GPIO 模拟PWD驱动电机
3. [x] i2c 驱动舵机（PCA9865）
4. [x] usb 摄像头启动
#### 传感器
1. [ ] 红外
2. [ ] 烟雾
3. [ ] 超声波测距
4. [ ] 火焰传感器
5. [ ] LED灯阵
## 车辆程序

#### 车辆程序

1. [ ] 主程序服务，接收控制程序信号与反馈
2. [ ] 电机控制程序，驱动车辆行动
   1. [ ] 前进
   2. [ ] 后退
   3. [ ] 左转
   4. [ ] 右转
   5. [ ] 寻迹智能行驶
   6. [ ] 自动避障行驶
   7. [ ] 路径规划行驶
3. [ ] 传感器功能
   1. [ ] 车辆状态检测试
   2. [ ] 车辆视频监控
   3. [ ] 车辆拍照
   4. [ ] 音乐播放
   5. [ ] 语音识別
   6. [ ] 氛围灯
4. [ ] 软件功能
   1. [ ] 文件传输
   2. [ ] 
## 控制程序
   1. [ ] 车辆连接 
      1. [ ] Qt
      2. [ ] QtApp
   2. [ ] 车辆状态
      1. [ ] Qt
      2. [ ] QtApp
   3. [ ] 车辆行驶
      1. [ ] Qt
      2. [ ] QtApp
      3. [ ] 红外
   4. [ ] 数据采集
      1. [ ] Qt
      2. [ ] QtApp
