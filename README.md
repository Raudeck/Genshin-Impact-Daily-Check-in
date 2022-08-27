# Genshin-Impact-Daily-Check-in
A rust application for hoyolab's genshin impact daily check-in
## Usage
```
git clone https://github.com/Raudeck/Genshin-Impact-Daily-Check-in.git
cd Genshin-Impact-Daily-Check-in
```
Modify src/main.rs to your account_id and cookie_token
```
let checkin = checkin::AccountInfo::new("account_id=YOUR_ACCOUNT_ID; cookie_token=YOUT_COOKIE_TOKEN".to_string());
```
Then
```
cargo build --release
```
