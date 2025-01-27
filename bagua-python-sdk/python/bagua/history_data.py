import boto3
from botocore import UNSIGNED
from botocore.config import Config


class HistoryData:
    client = None

    def __init__(self):
        self.client = boto3.client("s3", config=Config(signature_version=UNSIGNED))

    def get_files(self):
        req_token = None

        bucket = "hyperliquid-archive"
        prefix = "market_data/202307"

        is_finish = False

        while True:
            if req_token is None:
                response = self.client.list_objects_v2(Bucket=bucket, Prefix=prefix)
            else:
                response = self.client.list_objects_v2(Bucket=bucket, Prefix=prefix, ContinuationToken=req_token)

            for obj in response.get("Contents", []):
                key: str = obj["Key"]
                if key.endswith("BTC.lz4"):
                    is_finish = True
                    self.client.download_file(bucket, key, obj["ETag"] + ".lz4")
                    break
            if is_finish:
                break
            if response["IsTruncated"]:
                req_token = response["NextContinuationToken"]
            else:
                break


if __name__ == "__main__":
    HistoryData().get_files()
