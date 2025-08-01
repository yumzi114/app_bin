#!/bin/bash

# === 설정 부분 ===
TARGET="aarch64-unknown-linux-gnu"  # Raspberry Pi 4/5 (64-bit OS) 기준
# Pi OS 32bit면 armv7-unknown-linux-gnueabihf 로 변경
PI_USER="yum"
PI_HOST="192.168.0.138"     # 라즈베리파이 IP 주소
PI_PATH="/home/yum/yum_dir" 
BIN_PATH="/usr/bin"    
BIN_NAME="app_bin"          # Cargo 패키지명과 동일


echo "[1/3] Cross 빌드 시작..."
cross build --target $TARGET --release || { echo "빌드 실패!"; exit 1; }

# === 전송 (비밀번호 입력 필요add_space
echo "[2/3] SCP로 파일 전송 (비밀번호 입력)..."
ssh $PI_USER@$PI_HOST "sudo rm $BIN_PATH/$BIN_NAME"
scp target/$TARGET/release/$BIN_NAME $PI_USER@$PI_HOST:$PI_PATH

# === 실행 권한 부여 및 테스트 ===
echo "[3/3] SSH 접속 (비밀번호 입력 후 실행 권한 부여)..."

ssh $PI_USER@$PI_HOST "sudo chmod +x $PI_PATH/$BIN_NAME"
ssh $PI_USER@$PI_HOST "sudo cp $PI_PATH/$BIN_NAME $BIN_PATH"
ssh $PI_USER@$PI_HOST "sudo -S systemctl restart $BIN_NAME.service"


echo "✅ 배포 완료!"