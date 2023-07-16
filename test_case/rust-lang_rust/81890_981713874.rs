python
import github
import requests
from github import Github


def main():
    token = "<replace with token>"
    g = Github(token)
    # github.enable_console_debug_logging()
    repo_name = "rust-lang-ci/rust"
    repo = g.get_repo(repo_name)
    workflow_runs = repo.get_workflow_runs(status="failure", branch="auto")
    for run in workflow_runs:
        jobs_url = run.jobs_url + "?per_page=100"
        response = requests.get(jobs_url, headers={"Authorization": f"token {token}"})
        response.raise_for_status()
        for job in response.json()["jobs"]:
            if job["conclusion"] == "failure" and "msvc" in job["name"]:
                log_url = f"https://api.github.com/repos/{repo_name}/actions/jobs/{ job['id']}/logs"
                log_response = requests.get(
                    log_url, headers={"Authorization": f"token {token}"}
                )
                log_response.raise_for_status()
                log = log_response.text
                if "LNK1201" in log:
                    print(log_url)
                    print("\n\n")
                    print(log)
