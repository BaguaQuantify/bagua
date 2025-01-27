import boto3
from botocore import UNSIGNED
from botocore.config import Config


class HistoryData:
    client = None

    def __init__(self):
        self.client = boto3.client("s3", config=Config(signature_version=UNSIGNED))

    def get_files(self):
        req_token = None

        while True:
            if req_token is None:
                response = self.client.list_objects_v2(Bucket="hyperliquid-archive")
            else:
                response = self.client.list_objects_v2(Bucket="hyperliquid-archive", ContinuationToken=req_token)

            if "NextContinuationToken" in response:
                req_token = response["NextContinuationToken"]

            for obj in response.get("Contents", []):
                key: str = obj["Key"]
                if key.endswith("BTC.lz4"):
                    print(key)


if __name__ == "__main__":
    HistoryData().get_files()
