
% git branch backup master # just to save your work in case something goes wronge
% git reset --soft HEAD~ # this throws away commit 8662e95, but leaves the change you made (removing tcp/ip)
% git commit --amend # this adds the changes you made in 8662e95 to 44692ac
% gitk --all& # see if everything changed like you expected
% git branch -D backup # assuming everything looks good
% git push -f $YOUR_REMOTE master
