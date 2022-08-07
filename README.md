# lazyfinder

## 快速查询，极致性能


```
cargo build --release

example:
不启用正则，仅匹配key，用逗号隔开
./lazyfinder -d / -p ".py" -k "120.77.35.111,get_username(),Library/Logs/tmp" > result.txt
启用正则，最后加-r参数
./lazyfinder -d ~/  -p "txt" -k "\d+\.\d+\.\d+\.\d+" -r > result.txt
```

# 效果

<img width="964" alt="image" src="https://user-images.githubusercontent.com/38530231/153534773-b4bbc7b3-8e62-4ab7-949b-b0bb38aac126.png">
<img width="1069" alt="image" src="https://user-images.githubusercontent.com/38530231/155306639-ea8ccacc-9b3d-4dfe-8ed9-f703a72f3028.png">

