# Deploy new collector notes

```
sudo adduser --system --disabled-password --no-create-home monitor-collector
sudo mkdir -p /usr/local/lib/monitor
sudo setfacl -m user:alex:rwx /usr/local/lib/monitor
```

Copy systemd service `${REPO}/conf/monitor-collector.service` to
target at `/usr/local/lib/systemd/system/monitor-collector.service`
make sure it is owned by root and not world writable
```
sudo systemctl daemon-reload
sudo systemctl enable monitor-collector.service
sudo systemctl restart monitor-collector.service
sleep 1
sudo systemctl status monitor-collector.service
```

Put this in sudoers, replacing `$(hostname)`
```
# Allow alex to run, stop, or restart the monitor-collector service
alex $(hostname)=(root) NOPASSWD: /bin/systemctl restart monitor-collector.service, /bin/systemctl stop monitor-collector.service, /bin/systemctl start monitor-collector.service
```

Copy the cert and key to the server, set key permissions to read only by monitor-collector.

Now use `${REPO}/bin/deploy_collectors`
