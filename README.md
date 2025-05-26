# bds-listd-bridge
![image](https://github.com/user-attachments/assets/ecf3b9d2-3967-4f67-8977-70816af33c43)

This is an external application designed to enhance the functionality of the Bedrock Dedicated Server (BDS) by processing its `/listd` command output.  
It acts as a bridge, taking the raw listd log entries, processing them into a structured format, and then sending this formatted data back into the game environment using `/scriptevent` commands.  
This enables custom server features and interactions based on detailed player or server statistics.

## Requirements

To run `bds-listd-bridge`, you will need:

* **Bedrock Dedicated Server (BDS)**: https://www.minecraft.net/en-us/download/server/bedrock
* **Operating System**: Windows (specifically tested on Windows 10/11)
* **ScriptAPI**: To interact with `bds-listd-bridge` from within your BDS.

## License

This project is licensed under the [MIT License](LICENSE) .

## How to run

Download the latest `bds-listd-bridge.exe` from Releases.  
Place `bds_listd-bridge.exe` in the same directory as `bedrock_server.exe`.  
Launch `bds_listd-bridge.exe`.

## How to use

To utilize the features of this software within your BDS world, you'll need to use the ScriptAPI.  
We provide a dedicated library, `listdBridge.ts`, for this purpose.  
Please download it from the Releases page and integrate it into your ScriptAPI project.

## Inspirations

This project is inspired by and built upon the ideas from the following repository:

* [SKYNETWORK-MCBE/bds-enhancer](https://github.com/SKYNETWORK-MCBE/bds-enhancer)


