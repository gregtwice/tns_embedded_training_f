# Training for embedded systems with Renode

## Installing Renode
For Windows:  (need admin rights)
- go to [https://github.com/renode/renode/releases](https://github.com/renode/renode/releases) and download the latest version of the .msi installer. 

## Installing the ARM toolchain

Assuming you run a WSL machine

### From the package manager

```
sudo apt-get install gcc-arm-none-eabi
```

### From the sources 
Download the ARM toolchain

``` bash
wget -O gcc-arm-none-eabi.tar.xz https://developer.arm.com/-/media/Files/downloads/gnu/12.2.mpacbti-bet1/binrel/arm-gnu-toolchain-12.2.mpacbti-bet1-x86_64-arm-none-eabi.tar.xz
```

You can then install it in the folder of your choice and add the binaries to your path

``` bash
tar xf ./gcc-arm-none-eabi.tar.xz --strip-components=1 -C <FOLDER_PATH_HERE>
echo 'export PATH=$PATH:<FOLDER_PATH_HERE>/bin' >> ~/.bashrc
```

### Testing the toolchain

```
arm-none-eabi-gcc --version
arm-none-eabi-gdb --version
```

## Building the project

Prerequisites: (sudo apt install) 
- Cmake   
- Ninja

>**Note**
> 
>If the cmake version is not recent enough follow the instructions from this answer : https://askubuntu.com/a/1157132. to know your distribution : 
>`lsb_release -a`
``` bash
mkdir build && cd build
cmake -G "Ninja" -DCMAKE_TOOLCHAIN_FILE=../arm-none-eabi-gcc.cmake -DCMAKE_BUILD_TYPE=Debug ..
ninja
```

### Building a specific target

```bash
# Baremetal 
ninja Baremetal/all
# FreeRTOS
ninja FreeRTOS/all
```


## Testing the sample

If renode is installed on Windows, launch `renode-test` from a powershell or cmd prompt

``` powershell
renode-test "<Demo folder location>\test\test.robot"
```

**If you use WSL to compile** : Use "\\\\wsl$\\distro_name\\home\\... as a base for the path to the robot file