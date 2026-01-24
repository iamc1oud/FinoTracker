Perfect! **kind (Kubernetes IN Docker)** is actually great for local dev, lighter than Minikube, and works almost identically. We just need a few small adjustments for the setup. I’ll guide you step by step.

---

# 🚀 Step 1 — Using kind Instead of Minikube

---

## 1️⃣ Install & Verify kind

```bash
kind version
```

If not installed:

```bash
brew install kind   # macOS
# OR
curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.25.0/kind-linux-amd64
chmod +x ./kind
sudo mv ./kind /usr/local/bin/kind
```

---

## 2️⃣ Create a kind cluster

```bash
kind create cluster --name ai-expense-cluster --config ./infra/k8s/kind-config.yaml
```

Optional `kind-config.yaml` for multi-node dev:

```yaml
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
- role: worker
- role: worker
```

Verify:

```bash
kubectl get nodes
```

Should show 3 nodes (`control-plane` + 2 workers).

---

## 3️⃣ Load your Docker images into kind

kind **doesn’t see your local Docker images** by default, unlike Minikube. So after building the API Gateway image:

```bash
docker build -t api-gateway:latest ./services/api-gateway
kind load docker-image api-gateway:latest --name ai-expense-cluster
```

Now the kind cluster can deploy the image directly.

---

## 4️⃣ Apply K8s manifests

```bash
kubectl apply -f infra/k8s/base/deployment.yaml
kubectl apply -f infra/k8s/base/service.yaml
```

Verify pods:

```bash
kubectl get pods
```

Port-forward to test locally:

```bash
kubectl port-forward svc/api-gateway 8000:80
curl http://localhost:8000/health/
```

✅ Should return `{"status":"ok"}`

---

## 5️⃣ Secrets in kind

Same as before:

```bash
kubectl create secret generic crypto-secrets \
--from-literal=master-key="$(openssl rand -hex 32)"
```

The API Gateway pod will automatically pick it up via `env`.

---

## 6️⃣ Optional: Local ingress

If you want multiple services behind a **single gateway** (like real API Gateway + auth + core):

```bash
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/controller-v1.9.1/deploy/static/provider/kind/deploy.yaml
```

Then you can configure ingress rules to route:

```
/auth -> auth-service
/expenses -> core-service
```

---

✅ With this, **kind works exactly like Minikube** but lighter.

* You can run **multiple services**
* Use **real Docker images**
* Use **TDD and API Gateway** locally
