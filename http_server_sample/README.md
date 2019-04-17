# Example to use prometheus

## Performance result

### Cmd

* concurry: 100  -- no diffenence with the error ranges.

```
ab -c 100 -n 10000 http://127.0.0.1:9898/aaa
```

Enable Prometheus:
```
Server Hostname:        127.0.0.1
Server Port:            9891

Document Path:          /bbb
Document Length:        145 bytes

Concurrency Level:      100
Time taken for tests:   0.452 seconds
Complete requests:      10000
Failed requests:        5462
   (Connect: 0, Receive: 0, Length: 5462, Exceptions: 0)
Total transferred:      2403839 bytes
HTML transferred:       1443839 bytes
Requests per second:    22143.64 [#/sec] (mean)
Time per request:       4.516 [ms] (mean)
Time per request:       0.045 [ms] (mean, across all concurrent requests)
Transfer rate:          5198.22 [Kbytes/sec] received
```

Disable Prometheus:

```

Server Hostname:        127.0.0.1
Server Port:            9898

Document Path:          /bbb
Document Length:        144 bytes

Concurrency Level:      100
Time taken for tests:   0.461 seconds
Complete requests:      10000
Failed requests:        5149
   (Connect: 0, Receive: 0, Length: 5149, Exceptions: 0)
Total transferred:      2403791 bytes
HTML transferred:       1443791 bytes
Requests per second:    21674.58 [#/sec] (mean)
Time per request:       4.614 [ms] (mean)
Time per request:       0.046 [ms] (mean, across all concurrent requests)
Transfer rate:          5088.00 [Kbytes/sec] received
```

