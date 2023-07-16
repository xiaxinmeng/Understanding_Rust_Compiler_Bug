javascript
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.question('Your guess: ', (answer) => {
  const guess = parseInt(answer);
  rl.close();
});
