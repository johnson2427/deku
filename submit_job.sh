CONTRACT=$(jq -r .contract ./deku_contract/deployed.json)
YAML="[$(cat manifests/demo-pod.yaml | hexdump -v -e '1/1 "%d,"' | sed 's/,$//')]"
echo $CONTRACT
echo $YAML

pushd deku_contract
cargo contract call \
  --contract "$CONTRACT" \
  --message submit_job \
  --suri //Alice \
  --execute \
  -y \
  --args "$YAML" \
  --url ws://localhost:9944
popd
