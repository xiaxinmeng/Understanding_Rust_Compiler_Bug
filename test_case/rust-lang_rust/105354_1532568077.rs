
rustc +stage1  --print deployment-target
deployment_target=11.0

rustc +stage1 --target x86_64-apple-darwin --print deployment-target
deployment_target=10.7
