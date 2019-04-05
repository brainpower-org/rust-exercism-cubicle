#!/usr/bin/env node
const data = require('./exercises.json');

function main() {
    const output = data.exercises
        .filter(e => !e.deprecated)
        .map(e => `- [ ] ${e.slug} ${e.core ? '(core)' : ''} - ${e.difficulty}`);
    console.log(output.join('\n'));
}

main();