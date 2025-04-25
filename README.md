
### 🌸wobbwz 💗✨🌸  
> by purrbytes.sh 🐾 powered by snacks and spite

# 🐧💖 ~ wobblz ~ 💖🐧

*～ a smol babie fuzzer that waddlez into ur URLz and pokez them gently 🥺💕 ～*

made wif 💅 by [purrbytes.sh](https://purrbytes.sh) (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧  
powered by rust, curiosity, and a sprinkle of chaos 💫

---

## ✨ what is wobblz? ✨

wobblz is a **baby fuzzer** 💻💨  
she's ✨tiny✨  
she's 🍰cute🍰  
she will **scuttle into ur target web thingies** and test all the fuzzy bits you give her!! 🥹

> “hi!! i’m wobblz!! i’m gonna try all the urls… and maybe find secrets... heehee~ (´｡• ᵕ •｡`) ♡”

---

## 🌈 how do i use her?

you tell her where to wobble, and what to wobble wif! 🐧💼

```
wobblz -t "http://FUZZY.local:FUZZPORT/fuzzme" -w1 subbies.txt -w2 portsies.txt -w3 dirbies.txt
```

she gonna go:
```
✨ trying http://admin.local:8080/secret...
✨ trying http://test.local:3000/private...
✨ OMG! found sth 😳💥
```

---

## 🎀 features (so soft~)

- 💅 **you choose the fuzzwordz** like FUZZME, FUZZPORT, FUZZA etc.
- 🐾 **multi-fuzz support!** as many `-wN` wordlistz as your heart can handle
- 💌 **super duper fast** bc rust go brrrr 💨
- 🌸 **tiny & cute code**base (easy to fork, mod, hug)
- 💣 **good for hacking, not for cracking pls be kind 🙏**

---

## 🛠️ install me owo

```bash
git clone https://github.com/purrbytes/wobblz
cd wobblz
cargo build --release
./target/release/wobblz --help
```

> soon: `curl -sL install.wobblz.sh | bash` (but like… safer than it sounds… maybe...?)

---

## 🧃 example run!! (tehe~)

```bash
wobblz \
  -t "http://FUZZSUB.local:FUZZPORT/FUZZDIR" \
  -w1 subzies.txt \
  -w2 portzies.txt \
  -w3 dirzies.txt
```

she will **waddle thru all the fuzzies** and say:

```
(｡•́︿•̀｡)  wobblz iz wobbling… pls wait…
✨ tried 1800 comboz... found 3 spiky thingz! ✨
```

---

## 🧸 contribute?

YES PLS!! 🐾  
she loves frens!!!  
u can add features, make cursed forks (`wobblz.exe`, `wobblz+`, `evilwobblz`) or give her new outfits (ascii logos 👀)

---

## 🍓 license

MIT!! do whatever uwu just don’t make her sad ok (｡•́︿•̀｡)

---

## 💖 finale: ascii babie

```
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡤⠴⠒⠒⠒⠶⢤⣄⡀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣠⠞⠁⠀⠀⠀⠀⠀⠀⠀⠈⠙⢦⡀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⡼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢳⡀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢸⠁⠀⠀⣠⠖⠛⠛⠲⢤⠀⠀⠀⣰⠚⠛⢷⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣿⠀⠀⣸⠃⠀⠀⢀⣀⠈⢧⣠⣤⣯⢠⣤⠘⣆⠀⠀⠀< hehe url go boom
⠀⠀⠀⠀⠀⣿⠀⠀⡇⠀⠀⠀⠻⠟⠠⣏⣀⣀⣨⡇⠉⢀⣿⠀⠀⠀ 
⠀⠀⠀⠀⢀⡟⠀⠀⠹⡄⠀⠀⠀⠀⠀⠉⠑⠚⠉⠀⣠⡞⢿⠀⠀⠀   wobbwz
⠀⠀⠀⢀⡼⠁⠀⠀⠀⠙⠳⢤⡄⠀⠀⠀⠀⠀⠀⠀⠁⠙⢦⠳⣄⠀      by purrbytes.sh
⠀⠀⢀⡾⠁⠀⠀⠀⠀⠀⠤⣏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠃⠙⡆
⠀⠀⣼⠁⠀⠀⠀⠀⠀⠀⠀⠈⠳⣄⠀⠀⠀⠀⠀⠀⠀⢠⡏⠀⠀⡇
⠀⠀⣏⠀⠀⠀⠀⠲⣄⡀⠀⠀⠀⠸⡄⠀⠀⠀⠀⠀⠀⢸⠀⢀⡼⠁
⢀⡴⢿⠀⠀⠀⠀⠀⢸⠟⢦⡀⠀⢀⡇⠀⠀⠀⠀⠀⠀⠘⠗⣿⠁⠀
⠸⣦⡘⣦⠀⠀⠀⠀⣸⣄⠀⡉⠓⠚⠀⠀⠀⠀⠀⠀⠀⠀⡴⢹⣦⡀
⠀⠀⠉⠛⠳⢤⣴⠾⠁⠈⠟⠉⣇⠀⠀⠀⠀⠀⠀⠀⣠⠞⠁⣠⠞⠁
⠀⠀⠀⠀⠀⠀⠙⢧⣀⠀⠀⣠⠏⠀⠀⢀⣀⣠⠴⠛⠓⠚⠋⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠉⠙⠋⠉⠉⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀

```

> ✨ if she finds sth good, give her a snack or she’ll cry (ง'̀-'́)ง ✨

---

*created with luv & claws by purrbytes.sh ~ pls use responsibly or she gonna bite u 🦷💥*

