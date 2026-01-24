
1.1 Build & test locally

```bash
docker build -t api-gateway:latest .
docker run -p 8000:8000 --env MASTER_KEY=$(openssl rand -hex 32) api-gateway:latest
```

1.2 Load image to kind control plane if not present

```bash
kind load docker-image api-gateway:latest --name <cluster_name>
```


1.3 Deploy to K8s (Dev Overlay)

```bash
kubectl apply -f infra/k8s/base/deployment.yaml
kubectl apply -f infra/k8s/base/service.yaml
kubectl get pods
kubectl port-forward svc/api-gateway 8000:80
```
