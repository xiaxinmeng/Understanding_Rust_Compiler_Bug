plain
 ---> aa25d81a67cf
Step 10/14 : RUN pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt
 ---> Running in 79553c01ae1c
Collecting binaryornot==0.4.4 (from -r /tmp/reuse-requirements.txt (line 7))
  Retrying (Retry(total=4, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f99177dd208>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=3, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f99177dd780>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=2, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f9917f96128>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=1, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f99177cb0b8>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=0, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f99177cb748>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 144, in _new_conn
    (self.host, self.port), self.timeout, **extra_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/connection.py", line 83, in create_connection
    raise err
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/connection.py", line 73, in create_connection
    sock.connect(sa)
OSError: [Errno 101] Network is unreachable

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 601, in urlopen
    chunked=chunked)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 346, in _make_request
    self._validate_conn(conn)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 852, in _validate_conn
    conn.connect()
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 298, in connect
    conn = self._new_conn()
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 153, in _new_conn
    self, "Failed to establish a new connection: %s" % e)
urllib3.exceptions.NewConnectionError: <urllib3.connection.VerifiedHTTPSConnection object at 0x7f99177cb2e8>: Failed to establish a new connection: [Errno 101] Network is unreachable

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/adapters.py", line 440, in send
    timeout=timeout
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  [Previous line repeated 2 more times]
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 639, in urlopen
    _stacktrace=sys.exc_info()[2])
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/retry.py", line 398, in increment
    raise MaxRetryError(_pool, url, error or ResponseError(cause))
urllib3.exceptions.MaxRetryError: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl (Caused by NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f99177cb2e8>: Failed to establish a new connection: [Errno 101] Network is unreachable',))

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/lib/python3/dist-packages/pip/basecommand.py", line 215, in main
    status = self.run(options, args)
  File "/usr/lib/python3/dist-packages/pip/commands/install.py", line 353, in run
    wb.build(autobuilding=True)
  File "/usr/lib/python3/dist-packages/pip/wheel.py", line 749, in build
    self.requirement_set.prepare_files(self.finder)
  File "/usr/lib/python3/dist-packages/pip/req/req_set.py", line 380, in prepare_files
    ignore_dependencies=self.ignore_dependencies))
  File "/usr/lib/python3/dist-packages/pip/req/req_set.py", line 620, in _prepare_file
    session=self.session, hashes=hashes)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 821, in unpack_url
    hashes=hashes
  File "/usr/lib/python3/dist-packages/pip/download.py", line 659, in unpack_http_url
    hashes)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 876, in _download_http_url
    stream=True,
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 533, in get
    return self.request('GET', url, **kwargs)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 386, in request
    return super(PipSession, self).request(method, url, *args, **kwargs)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 520, in request
    resp = self.send(prep, **send_kwargs)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 630, in send
    r = adapter.send(request, **kwargs)
  File "/usr/share/python-wheels/CacheControl-0.11.7-py2.py3-none-any.whl/cachecontrol/adapter.py", line 47, in send
    resp = super(CacheControlAdapter, self).send(request, **kw)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/adapters.py", line 508, in send
    raise ConnectionError(e, request=request)
requests.exceptions.ConnectionError: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl (Caused by NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f99177cb2e8>: Failed to establish a new connection: [Errno 101] Network is unreachable',))
The command '/bin/sh -c pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt' returned a non-zero code: 2
Sending build context to Docker daemon  521.2kB

Step 1/14 : FROM ubuntu:18.04
 ---> 476c64862aa8
---
 ---> aa25d81a67cf
Step 10/14 : RUN pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt
 ---> Running in 6ebb3b27c5c2
Collecting binaryornot==0.4.4 (from -r /tmp/reuse-requirements.txt (line 7))
  Retrying (Retry(total=4, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7feb018e1320>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=3, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7feb018e1048>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=2, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7feb018e1be0>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=1, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7feb018cf160>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=0, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7feb018cf9e8>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 144, in _new_conn
    (self.host, self.port), self.timeout, **extra_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/connection.py", line 83, in create_connection
    raise err
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/connection.py", line 73, in create_connection
    sock.connect(sa)
OSError: [Errno 101] Network is unreachable

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 601, in urlopen
    chunked=chunked)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 346, in _make_request
    self._validate_conn(conn)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 852, in _validate_conn
    conn.connect()
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 298, in connect
    conn = self._new_conn()
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 153, in _new_conn
    self, "Failed to establish a new connection: %s" % e)
urllib3.exceptions.NewConnectionError: <urllib3.connection.VerifiedHTTPSConnection object at 0x7feb02099cc0>: Failed to establish a new connection: [Errno 101] Network is unreachable

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/adapters.py", line 440, in send
    timeout=timeout
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  [Previous line repeated 2 more times]
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 639, in urlopen
    _stacktrace=sys.exc_info()[2])
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/retry.py", line 398, in increment
    raise MaxRetryError(_pool, url, error or ResponseError(cause))
urllib3.exceptions.MaxRetryError: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl (Caused by NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7feb02099cc0>: Failed to establish a new connection: [Errno 101] Network is unreachable',))

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/lib/python3/dist-packages/pip/basecommand.py", line 215, in main
    status = self.run(options, args)
  File "/usr/lib/python3/dist-packages/pip/commands/install.py", line 353, in run
    wb.build(autobuilding=True)
  File "/usr/lib/python3/dist-packages/pip/wheel.py", line 749, in build
    self.requirement_set.prepare_files(self.finder)
  File "/usr/lib/python3/dist-packages/pip/req/req_set.py", line 380, in prepare_files
    ignore_dependencies=self.ignore_dependencies))
  File "/usr/lib/python3/dist-packages/pip/req/req_set.py", line 620, in _prepare_file
    session=self.session, hashes=hashes)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 821, in unpack_url
    hashes=hashes
  File "/usr/lib/python3/dist-packages/pip/download.py", line 659, in unpack_http_url
    hashes)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 876, in _download_http_url
    stream=True,
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 533, in get
    return self.request('GET', url, **kwargs)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 386, in request
    return super(PipSession, self).request(method, url, *args, **kwargs)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 520, in request
    resp = self.send(prep, **send_kwargs)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 630, in send
    r = adapter.send(request, **kwargs)
  File "/usr/share/python-wheels/CacheControl-0.11.7-py2.py3-none-any.whl/cachecontrol/adapter.py", line 47, in send
    resp = super(CacheControlAdapter, self).send(request, **kw)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/adapters.py", line 508, in send
    raise ConnectionError(e, request=request)
requests.exceptions.ConnectionError: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl (Caused by NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7feb02099cc0>: Failed to establish a new connection: [Errno 101] Network is unreachable',))
The command '/bin/sh -c pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt' returned a non-zero code: 2
Sending build context to Docker daemon  521.2kB

Step 1/14 : FROM ubuntu:18.04
 ---> 476c64862aa8
---
 ---> aa25d81a67cf
Step 10/14 : RUN pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt
 ---> Running in e4895e7230a9
Collecting binaryornot==0.4.4 (from -r /tmp/reuse-requirements.txt (line 7))
  Retrying (Retry(total=4, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f31a4dec390>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=3, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f31a4dec240>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=2, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f31a4ddc0f0>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=1, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f31a4ddc9b0>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=0, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f31a4ddc710>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 144, in _new_conn
    (self.host, self.port), self.timeout, **extra_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/connection.py", line 83, in create_connection
    raise err
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/connection.py", line 73, in create_connection
    sock.connect(sa)
OSError: [Errno 101] Network is unreachable

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 601, in urlopen
    chunked=chunked)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 346, in _make_request
    self._validate_conn(conn)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 852, in _validate_conn
    conn.connect()
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 298, in connect
    conn = self._new_conn()
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 153, in _new_conn
    self, "Failed to establish a new connection: %s" % e)
urllib3.exceptions.NewConnectionError: <urllib3.connection.VerifiedHTTPSConnection object at 0x7f31a55a5c50>: Failed to establish a new connection: [Errno 101] Network is unreachable

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/adapters.py", line 440, in send
    timeout=timeout
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  [Previous line repeated 2 more times]
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 639, in urlopen
    _stacktrace=sys.exc_info()[2])
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/retry.py", line 398, in increment
    raise MaxRetryError(_pool, url, error or ResponseError(cause))
urllib3.exceptions.MaxRetryError: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl (Caused by NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f31a55a5c50>: Failed to establish a new connection: [Errno 101] Network is unreachable',))

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/lib/python3/dist-packages/pip/basecommand.py", line 215, in main
    status = self.run(options, args)
  File "/usr/lib/python3/dist-packages/pip/commands/install.py", line 353, in run
    wb.build(autobuilding=True)
  File "/usr/lib/python3/dist-packages/pip/wheel.py", line 749, in build
    self.requirement_set.prepare_files(self.finder)
  File "/usr/lib/python3/dist-packages/pip/req/req_set.py", line 380, in prepare_files
    ignore_dependencies=self.ignore_dependencies))
  File "/usr/lib/python3/dist-packages/pip/req/req_set.py", line 620, in _prepare_file
    session=self.session, hashes=hashes)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 821, in unpack_url
    hashes=hashes
  File "/usr/lib/python3/dist-packages/pip/download.py", line 659, in unpack_http_url
    hashes)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 876, in _download_http_url
    stream=True,
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 533, in get
    return self.request('GET', url, **kwargs)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 386, in request
    return super(PipSession, self).request(method, url, *args, **kwargs)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 520, in request
    resp = self.send(prep, **send_kwargs)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 630, in send
    r = adapter.send(request, **kwargs)
  File "/usr/share/python-wheels/CacheControl-0.11.7-py2.py3-none-any.whl/cachecontrol/adapter.py", line 47, in send
    resp = super(CacheControlAdapter, self).send(request, **kw)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/adapters.py", line 508, in send
    raise ConnectionError(e, request=request)
requests.exceptions.ConnectionError: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl (Caused by NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f31a55a5c50>: Failed to establish a new connection: [Errno 101] Network is unreachable',))
The command '/bin/sh -c pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt' returned a non-zero code: 2
Sending build context to Docker daemon  521.2kB

Step 1/14 : FROM ubuntu:18.04
 ---> 476c64862aa8
---
 ---> aa25d81a67cf
Step 10/14 : RUN pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt
 ---> Running in 4d0a4812f917
Collecting binaryornot==0.4.4 (from -r /tmp/reuse-requirements.txt (line 7))
  Retrying (Retry(total=4, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f937551c0b8>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=3, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f937551c940>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=2, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f937551c198>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=1, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f93755090f0>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=0, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f9375509908>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 144, in _new_conn
    (self.host, self.port), self.timeout, **extra_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/connection.py", line 83, in create_connection
    raise err
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/connection.py", line 73, in create_connection
    sock.connect(sa)
OSError: [Errno 101] Network is unreachable

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 601, in urlopen
    chunked=chunked)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 346, in _make_request
    self._validate_conn(conn)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 852, in _validate_conn
    conn.connect()
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 298, in connect
    conn = self._new_conn()
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 153, in _new_conn
    self, "Failed to establish a new connection: %s" % e)
urllib3.exceptions.NewConnectionError: <urllib3.connection.VerifiedHTTPSConnection object at 0x7f9375cd4c50>: Failed to establish a new connection: [Errno 101] Network is unreachable

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/adapters.py", line 440, in send
    timeout=timeout
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  [Previous line repeated 2 more times]
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 639, in urlopen
    _stacktrace=sys.exc_info()[2])
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/retry.py", line 398, in increment
    raise MaxRetryError(_pool, url, error or ResponseError(cause))
urllib3.exceptions.MaxRetryError: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl (Caused by NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f9375cd4c50>: Failed to establish a new connection: [Errno 101] Network is unreachable',))

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/lib/python3/dist-packages/pip/basecommand.py", line 215, in main
    status = self.run(options, args)
  File "/usr/lib/python3/dist-packages/pip/commands/install.py", line 353, in run
    wb.build(autobuilding=True)
  File "/usr/lib/python3/dist-packages/pip/wheel.py", line 749, in build
    self.requirement_set.prepare_files(self.finder)
  File "/usr/lib/python3/dist-packages/pip/req/req_set.py", line 380, in prepare_files
    ignore_dependencies=self.ignore_dependencies))
  File "/usr/lib/python3/dist-packages/pip/req/req_set.py", line 620, in _prepare_file
    session=self.session, hashes=hashes)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 821, in unpack_url
    hashes=hashes
  File "/usr/lib/python3/dist-packages/pip/download.py", line 659, in unpack_http_url
    hashes)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 876, in _download_http_url
    stream=True,
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 533, in get
    return self.request('GET', url, **kwargs)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 386, in request
    return super(PipSession, self).request(method, url, *args, **kwargs)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 520, in request
    resp = self.send(prep, **send_kwargs)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 630, in send
    r = adapter.send(request, **kwargs)
  File "/usr/share/python-wheels/CacheControl-0.11.7-py2.py3-none-any.whl/cachecontrol/adapter.py", line 47, in send
    resp = super(CacheControlAdapter, self).send(request, **kw)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/adapters.py", line 508, in send
    raise ConnectionError(e, request=request)
requests.exceptions.ConnectionError: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl (Caused by NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f9375cd4c50>: Failed to establish a new connection: [Errno 101] Network is unreachable',))
The command '/bin/sh -c pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt' returned a non-zero code: 2
Sending build context to Docker daemon  521.2kB

Step 1/14 : FROM ubuntu:18.04
 ---> 476c64862aa8
---
 ---> aa25d81a67cf
Step 10/14 : RUN pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt
 ---> Running in f92c922c26a6
Collecting binaryornot==0.4.4 (from -r /tmp/reuse-requirements.txt (line 7))
  Retrying (Retry(total=4, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f5577d083c8>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=3, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f5577d08320>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=2, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f5577d08048>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=1, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f5577cf6978>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
  Retrying (Retry(total=0, connect=None, read=None, redirect=None, status=None)) after connection broken by 'NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f5577cf6160>: Failed to establish a new connection: [Errno 101] Network is unreachable',)': /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 144, in _new_conn
    (self.host, self.port), self.timeout, **extra_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/connection.py", line 83, in create_connection
    raise err
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/connection.py", line 73, in create_connection
    sock.connect(sa)
OSError: [Errno 101] Network is unreachable

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 601, in urlopen
    chunked=chunked)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 346, in _make_request
    self._validate_conn(conn)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 852, in _validate_conn
    conn.connect()
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 298, in connect
    conn = self._new_conn()
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connection.py", line 153, in _new_conn
    self, "Failed to establish a new connection: %s" % e)
urllib3.exceptions.NewConnectionError: <urllib3.connection.VerifiedHTTPSConnection object at 0x7f5577cf6860>: Failed to establish a new connection: [Errno 101] Network is unreachable

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/adapters.py", line 440, in send
    timeout=timeout
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 668, in urlopen
    **response_kw)
  [Previous line repeated 2 more times]
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/connectionpool.py", line 639, in urlopen
    _stacktrace=sys.exc_info()[2])
  File "/usr/share/python-wheels/urllib3-1.22-py2.py3-none-any.whl/urllib3/util/retry.py", line 398, in increment
    raise MaxRetryError(_pool, url, error or ResponseError(cause))
urllib3.exceptions.MaxRetryError: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl (Caused by NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f5577cf6860>: Failed to establish a new connection: [Errno 101] Network is unreachable',))

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/lib/python3/dist-packages/pip/basecommand.py", line 215, in main
    status = self.run(options, args)
  File "/usr/lib/python3/dist-packages/pip/commands/install.py", line 353, in run
    wb.build(autobuilding=True)
  File "/usr/lib/python3/dist-packages/pip/wheel.py", line 749, in build
    self.requirement_set.prepare_files(self.finder)
  File "/usr/lib/python3/dist-packages/pip/req/req_set.py", line 380, in prepare_files
    ignore_dependencies=self.ignore_dependencies))
  File "/usr/lib/python3/dist-packages/pip/req/req_set.py", line 620, in _prepare_file
    session=self.session, hashes=hashes)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 821, in unpack_url
    hashes=hashes
  File "/usr/lib/python3/dist-packages/pip/download.py", line 659, in unpack_http_url
    hashes)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 876, in _download_http_url
    stream=True,
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 533, in get
    return self.request('GET', url, **kwargs)
  File "/usr/lib/python3/dist-packages/pip/download.py", line 386, in request
    return super(PipSession, self).request(method, url, *args, **kwargs)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 520, in request
    resp = self.send(prep, **send_kwargs)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/sessions.py", line 630, in send
    r = adapter.send(request, **kwargs)
  File "/usr/share/python-wheels/CacheControl-0.11.7-py2.py3-none-any.whl/cachecontrol/adapter.py", line 47, in send
    resp = super(CacheControlAdapter, self).send(request, **kw)
  File "/usr/share/python-wheels/requests-2.18.4-py2.py3-none-any.whl/requests/adapters.py", line 508, in send
    raise ConnectionError(e, request=request)
requests.exceptions.ConnectionError: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl (Caused by NewConnectionError('<urllib3.connection.VerifiedHTTPSConnection object at 0x7f5577cf6860>: Failed to establish a new connection: [Errno 101] Network is unreachable',))
The command '/bin/sh -c pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt' returned a non-zero code: 2
##[error]Process completed with exit code 1.
Post job cleanup.
