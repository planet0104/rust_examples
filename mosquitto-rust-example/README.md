# mosquitto-rust-example
 mosquitto rust example

## Docker安装mosquitto

```yml
# 映射当前目录下的mosquitto文件夹
services:
    mosquitto:
        image: eclipse-mosquitto
        volumes:
        - ./mosquitto/:/mosquitto/:rw
        ports:
        - 8883:8883
        - 9001:9001
```
```bash
docker-compose up -d mosquitto
```

## paho-mqtt编译配置

```bash
sudo apt install libssl-dev
sudo apt install clang
sudo apt install mosquitto-dev
```