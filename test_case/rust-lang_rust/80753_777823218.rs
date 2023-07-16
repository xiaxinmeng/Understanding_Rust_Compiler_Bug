
import boto3
import botocore
import csv
import json
import os
import collections
from concurrent.futures import ThreadPoolExecutor, as_completed

def mks3():
    session = boto3.Session(profile_name='mfa-rust')
    s3 = session.client('s3')
    return s3

status = json.load(open('progress.json'))
unsaved = 0
def save():
    global status
    print('Saving')
    json.dump(status, open('progress.tmp.json', 'w'))
    os.replace('progress.tmp.json', 'progress.json')
def set_deleted(key):
    global status
    global unsaved
    status[key] = True
    unsaved += 1
    if unsaved > 100:
        unsaved = 0
        save()

BUCKET = None
BATCH_SIZE = 100
NUM_WORKERS = 5
batches = []

def handle_batch(keys):
    global BUCKET
    s3 = mks3()
    to_delete = []
    deleted = []
    for key in keys:
        try:
            obj = s3.head_object(Bucket=BUCKET, Key=key)
            to_delete.append({ 'Key': key, 'VersionId': obj['VersionId'] })
        except botocore.exceptions.ClientError as e:
            if e.response['Error']['Code'] != '404':
                print('Failed to get object {} deets: {}'.format(key, e))
            else:
                print('Object {} missing, skipping'.format(key))
                deleted.append(key)
    if len(to_delete) != 0:
        print('Deleting {} objects'.format(len(to_delete)))
        ret = s3.delete_objects(Bucket=BUCKET, Delete={ 'Objects': to_delete })
        for obj in ret['Deleted']:
            deleted.append(obj['Key'])
        if 'Errors' in ret and len(ret['Errors']) != 0:
            print(ret['Errors'])
            assert False
    print('Finished deletion')
    return deleted

batch = []
rdr = csv.reader(open('nightlygzs'))
print('Creating batches')
for i, row in enumerate(rdr):
    if BUCKET is None:
        BUCKET = row[0]
    assert BUCKET == row[0]
    if row[1] in status:
        continue
    if len(batch) >= BATCH_SIZE:
        batches.append(batch)
        batch = []
    batch.append(row[1])

print('Got {} batches'.format(len(batches)))
executor = ThreadPoolExecutor(max_workers=NUM_WORKERS)
futures = [executor.submit(handle_batch, batch) for batch in batches]
print('Waiting for batch completion')
for i, complete in enumerate(as_completed(futures)):
    print('Completed batch {} of {}'.format(i+1, len(futures)))
    ret = complete.result()
    for key in ret:
        set_deleted(key)

save()
