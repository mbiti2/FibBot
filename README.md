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
      uses: mbiti2/FibBot@v1
      with: 
        enable_fib: true
          max_threshold: 150
          request_number: ${{ github.event.pull_request.number }}
          github_token: ${{ secrets.GITHUB_TOKEN }} 
          actor: ${{ github.actor }}
          repository:   ${{ github.repository }}
```
####### Enable workflow permissions
- Go to the repository setting
- Click on Actions
- Click on general
- Go to Workflow permissions and enable Read and write permissions 
### Testing
- Create a pull request and monitor the logs to see the processes that are running
- When all actions pass go to pull request comment section and see results
  


