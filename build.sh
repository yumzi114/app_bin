
#!/bin/bash

# 사용법: ./deploy.sh <PASSWORD>
# 예: ./deploy.sh mypassword

if [ $# -ne 1 ]; then
    echo "사용법: $0 <PASSWORD>"
    exit 1
fi

PASSWORD="$1"

# === 설정 부분 ===
TARGET="aarch64-unknown-linux-gnu"  # Pi OS 64bit 기준
PI_USER="yum"
PI_HOST="192.168.0.138"
PI_PATH="/home/yum/yum_dir"
BIN_PATH="/usr/bin"
BIN_NAME="app_bin"
ASSETS="assets"

echo "[1/3] Cross 빌드 시작..."
cross build --target $TARGET --release || { echo "빌드 실패!"; exit 1; }

# === 기존 파일 삭제 ===
echo "[2/3] 기존 파일 제거..."
sshpass -p "$PASSWORD" ssh $PI_USER@$PI_HOST "echo \"$PASSWORD\" | sudo -S rm -f $BIN_PATH/$BIN_NAME"

# === 새 파일 전송 ===
echo "[3/3] SCP로 새 파일 전송..."
sshpass -p "$PASSWORD" scp target/$TARGET/release/$BIN_NAME $PI_USER@$PI_HOST:$PI_PATH
sshpass -p "$PASSWORD" scp -r $ASSETS $PI_USER@$PI_HOST:$PI_PATH

# === 실행 권한 부여 및 배포 ===
echo "[4/4] SSH 접속 후 권한 부여 및 서비스 재시작..."
sshpass -p "$PASSWORD" ssh $PI_USER@$PI_HOST <<EOF
echo "$PASSWORD" | sudo -S chmod +x $PI_PATH/$BIN_NAME
echo "$PASSWORD" | sudo -S cp $PI_PATH/$BIN_NAME $BIN_PATH
echo "$PASSWORD" | systemctl --user restart $BIN_NAME.service
EOF
# ssh -t $PI_USER@$PI_HOST "systemctl --user restart app_bin.service"

echo "✅ 배포 완료!"
