FibBot - A GitHub Action for Fibonacci Calculation

## Introduction

FibBot is a GitHub Action that scans pull request descriptions for numbers, calculates their Fibonacci values, and posts a comment with the results.

### How It Works

- FibBot scans the pull request description for numbers.
- It calculates the Fibonacci sequence for each detected number.
- It posts a comment on the PR with the results.
  ### HOw to use this Action
##### Add this to yours workflow file
```  steps:
    - name: Compute Fibbot
      uses: @marcjazz/ticket-assistant@v1
      with: 
        
```


