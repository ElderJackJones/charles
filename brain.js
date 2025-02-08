import { exec } from 'child_process'
import robot from 'robotjs'
import loading from 'loading-cli'

// This program is basically the ghost of 
// Elder Jones, reanimated for 
// your convenience. Enjoy.
// It works, so don't touch it 
// unless you understand what you're doing

// open holly-master
function inflictHolly(path, command) {
    const load = loading('booting up holly').start()
    robot.typeString(`cd ${path}`)
    robot.keyTap('enter')
    load.text = 'opening folders'
    setTimeout(() => {
        load.text = 'starting holly'
        robot.typeString(command)
        robot.keyTap('enter')
        load.succeed('holly listening')
    }, 1000)
}

inflictHolly('holly-master', 'cargo run --release')