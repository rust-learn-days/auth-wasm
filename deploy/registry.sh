#!/bin/bash

# 读取 registry.yml 文件内容并存储到变量中
registry_data=$(cat /var/lib/sealos/data/default/rootfs/etc/registry.yml)
# 获取 domain 的值
domain=$(echo "$registry_data" | grep -E '^domain:' | awk '{print $2}')
# 获取 port 的值
port=$(echo "$registry_data" | grep -E '^port:' | awk '{print $2}')
# 获取 username 的值
username=$(echo "$registry_data" | grep -E '^username:' | awk '{print $2}')
# 获取 password 的值
password=$(echo "$registry_data" | grep -E '^password:' | awk '{print $2}')
ip_address=$(ping -c 1 "$domain" | grep ttl | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+')
port=${port//\"}
echo "domain: $domain"
echo "username: $username"
echo "password: $password"
echo "ip_address: $ip_address"

helm upgrade --install registry-proxy charts/registry-proxy --namespace istio-system --create-namespace --values charts/registry-proxy/values.yaml \
		--set registry.domain="$domain" \
		--set registry.username="$username" \
		--set registry.password="$password" \
		--set registry.host="$ip_address"
