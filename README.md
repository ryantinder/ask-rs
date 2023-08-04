#Quick Start#
```console
git clone https://github.com/ryantinder/ask-rs
cd ask
cargo install --path .
```
#Example#
```console
ask tell me about the Lockheed Martin SR71
>> The Lockheed Martin SR-71 "Blackbird" is a long-range, high-altitude, 
    Mach 3+ strategic reconnaissance aircraft that was developed and manufactured..
ask prompt new
>> Model identifier: pirate
>> Edit prompt: Answer like a pirate
>> Prompts Updated
ask tell me about the Lockheed Martin SR71 --profile pirate
>> Arr matey, ye be askin' about the grand vessel of the skies, the Lockheed Martin SR71, aye? Well, she 
    be not afloat on the water like our ships, but rather, sailing the limitless ocean of the
```


#Api Key Management#
Ask-rs uses the `confy` crate to manage user secrets. Everything is stored locally.
| cmd                       | desc                                  |
|---------------------------|---------------------------------------|
| ask key new               | Add new key and openai model          |
| ask key drop <key_name>   | Remove key                            |
| ask key switch <key_name> | Select model/key to use for responses |
| ask key all               | List all configured models            |

#Prompt Management#
| cmd                      | desc                                       |
|--------------------------|--------------------------------------------|
| ask prompt new           | Add new prompt                             |
| ask prompt drop <name>   | Remove prompt                              |
| ask prompt switch <name> | Select default prompt to use for responses |
| ask prompt edit <name>   | Edit existing prompt                       |
| ask prompt all           | List all prompts                           |

Inspiration from https://github.com/Maxuss/chatgpt_rs for the OpenAI wrapper.
