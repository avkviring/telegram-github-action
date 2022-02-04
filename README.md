## Setup

- add file telegram.yml to .github/workflows
- add secrets (telegram_to, telegram_token)
- enjoy

**.github/workflows/telegram.yml**
```
name: telegram

on:
  push:
    branches: [ master ]  
 release:    
    types:
      - published    
jobs:
  build:    
    runs-on: ubuntu-latest    
    steps:        
    - uses: avkviring/telegram-github-action@v0.0.17
      env:
        telegram_to: ${{ secrets.telegram_to }}  
        telegram_token: ${{ secrets.telegram_token }}
        event: ${{ toJson(github.event) }}
```

**Result in telegram**
```
user push to repo
 ➞ Commit message 1
 ➞ Commit message 2
 ➞ Commit message 3
``` 

```
user release repo@tag
Release title
```
