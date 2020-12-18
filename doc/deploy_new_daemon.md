# Deploy new daemon notes

## Deploy new monitor-collector

```
sudo adduser --system --disabled-password --no-create-home monitor-collector
sudo mkdir -p /usr/local/lib/monitor
sudo setfacl -m user:alex:rwx /usr/local/lib/monitor
```

Copy systemd service `${REPO}/conf/monitor-collector.service` to
target at `/usr/local/lib/systemd/system/monitor-collector.service`
make sure it is owned by root and not world writable:
```
sudo chown root:root monitor-collector.service
```

Enable the job in systemd:
```
sudo systemctl daemon-reload
sudo systemctl enable monitor-collector.service
sudo systemctl restart monitor-collector.service
sleep 1
sudo systemctl status monitor-collector.service
```

Put this in sudoers, replacing `$(hostname)`. Use `sudo visudo`.
```
# Allow alex to run, stop, or restart the monitor-collector service
alex $(hostname)=(root) NOPASSWD: /bin/systemctl restart monitor-collector.service, /bin/systemctl stop monitor-collector.service, /bin/systemctl start monitor-collector.service
```

Copy the cert and key to the server, set key permissions to read only by monitor-collector.
```
scp ${REPO}/cert/ok/{mf.fullchain,mf.key} mf:/usr/local/lib/monitor/cert/

sudo chown monitor-collector:root /usr/local/lib/monitor/cert/mf.key
sudo chmod 0400 /usr/local/lib/monitor/cert/mf.key
```

Now use `${REPO}/bin/deploy-collectors`

## Deploy new monitor-web

Very similar:

* Create new monitor-web user
* If required create /usr/local/lib/monitor and add write permissions for your user
* Copy systemd service monitor-web.service to target and set permissions
* Enable the job in systemd
* Set sudoers for your user to start, stop, restart the monitor-web.service
* Copy the cert and key to the server if required, set permissions
* If key has owner monitor-collector, add read permissions for monitor-web using `sudo setfacl -m user:monitor-web:r ${keyfile}`

Now use `${REPO}/bin/deploy-web`
