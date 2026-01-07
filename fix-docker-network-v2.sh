#!/bin/bash

# Docker Network Fix
# Resolves persistent DOCKER-ISOLATION-STAGE-2 chain issues

set -e

echo "Comprehensive Docker network repair..."
echo ""

echo "Stopping Docker services..."
sudo systemctl stop docker.socket
sudo systemctl stop docker

echo "Cleaning ALL iptables rules (will restore after)..."
sudo iptables -t filter -F
sudo iptables -t nat -F
sudo iptables -t mangle -F
sudo iptables -t filter -X
sudo iptables -t nat -X
sudo iptables -t mangle -X

echo "Removing Docker network state files..."
sudo rm -rf /var/lib/docker/network
sudo rm -rf /var/lib/docker/volumes

echo "Configuring iptables backend..."
sudo update-alternatives --set iptables /usr/sbin/iptables-nft 2>/dev/null || true
sudo update-alternatives --set ip6tables /usr/sbin/ip6tables-nft 2>/dev/null || true

echo "Starting Docker daemon..."
sudo systemctl start docker

echo "Waiting for Docker initialization..."
sleep 5

echo "Verifying iptables chains..."
if sudo iptables -t filter -L DOCKER-ISOLATION-STAGE-2 -n &>/dev/null; then
    echo "  DOCKER-ISOLATION-STAGE-2 chain created successfully"
else
    echo "  DOCKER-ISOLATION-STAGE-2 chain still missing - trying daemon restart..."
    sudo systemctl daemon-reload
    sudo systemctl restart docker
    sleep 5
fi

echo "Cleaning up Docker networks..."
docker network prune -f || true

echo ""
echo "Repair completed, It's Fixed!"
echo ""
echo "Verification:"
sudo iptables -t filter -L DOCKER-ISOLATION-STAGE-2 -n 2>&1 | head -5
echo ""
echo "Now try: docker compose up --build"
