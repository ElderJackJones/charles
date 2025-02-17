import { exec } from 'node:child_process';
import { promisify } from 'node:util';
import readline from 'readline';
import { writeFileSync, readFileSync, existsSync } from 'fs';

const promisedExec = promisify(exec);

function writeConfig(master, commander) {
    try {
      const configObj = {
        "holly-master": master,
        "referral_list": commander,
      };
      writeFileSync('config.json', JSON.stringify(configObj, null, 2)); 
      return true;
    } catch (e) {
      console.error(`An error occurred while saving the paths: ${e}`);
      return false;
    }
  }

async function findPath(name) {
  try {
    const { stdout } = await promisedExec(
      `cd C:\\; $File=Get-ChildItem ${name} -Recurse -ErrorAction SilentlyContinue; $FileLocation=$File.DirectoryName; $FileLocation`,
      { shell: 'powershell.exe' }
    );
    return stdout.trim();
  } catch (error) {
    console.error(`exec error: ${error}`);
    return null;
  }
}

async function savePath() {
    const response = readFileSync('config.json', 'utf-8');
    let config = await JSON.parse(response.trim())

  if (existsSync('config.json')) {
    console.log('premade path detected');
    console.log(JSON.stringify(config))
    return config;
  } else {
    const stream = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    stream.question(
      'Create bat file to save file paths for future startups? (y/n) ',
      async (answer) => {
        if (answer.toLowerCase() === 'y') {
          console.log('Searching for path...');
          const master = await findPath('holly.py');
          console.log('holly.py path:', master);
          const commander = await findPath('target.json'); // empty file for target practice
          console.log('target.json path:', commander);

          // save
          writeConfig(master, commander)
        } else {
          console.log('Searching for path...');
          const master = await findPath('holly.py');
          const commander = await findPath('target.json');
          console.log('holly.py path:', master);
          console.log('target.json path:', commander);
        }
        stream.close(); 
      }
    );
  }
}

savePath();
