#!/bin/bash

cd ..
echo "------------------------"
echo "k8s Cluster init ..."
kind create cluster --name insight-hoot --config ./kind/k8s_config/kind-config.yaml



# echo "Pass"
helm repo add spark-operator https://kubeflow.github.io/spark-operator
echo "------------------------"
echo "Install  Insight-hoot helm Chart "
helm install i-h ./kind/insight-hoot-chart/
echo "------------------------"
echo "Install  psark operator helm Chart "
helm install my-release spark-operator/spark-operator --version 1.2.7 --namespace spark-operator --create-namespace --set webhook.enable=true --debug



echo "------------------------"
echo "Spark Application"
kubectl apply -f ./kind/k8s_config/spark-pi.yaml

