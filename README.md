## Kuberenetes Infra Setup
1.1 Install Dependencies

```bash
kubectl version --client
kind version
helm version
```

1.2 Secrets (KMS/Master Key)
```bash
kubectl create secret generic crypto-secrets --from-literal=master-key="$(openssl rand -hex 32)"
```
