import json

class ConfigMaker:
    def __init__(self):
        print("Remote Control Solenoid Config Maker!\n")

    def write_config(self,data_dict):
        with open("config.json" , "w") as File:
            data_to_write =json.dumps(data_dict)
            File.write(data_to_write)

    def create_config(self):
        host = input("host:")
        port = input("port:")
        name = input("name:")
        data_dict = {"host":host, "port":port, "name":name}
        self.write_config(data_dict)
        self.logger.log_config_success()

if __name__ == "__main__":
    ConfigMaker().create_config()