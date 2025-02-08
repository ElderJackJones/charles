import { exec } from 'child_process'
import robot from 'robotjs'
import loading from 'loading-cli'

// This program is basically the ghost of 
// Elder Jones, reanimated for 
// your convenience. Enjoy.
// It works, so don't touch it 
// unless you understand what you're doing

// open holly-master
function inflictCharles(path, command, swirl, process) {
    swirl(process, 'opening secret doors...')
    setTimeout(() => {
        swirl(process, "please don't touch any keys")
    }, 1000)

    // switching to new tab
    robot.keyToggle('control', 'down')
    robot.keyToggle('shift', 'down')
    robot.keyTap('t')
    robot.keyToggle('control', 'up')
    robot.keyToggle('shift', 'up')
    robot.keyToggle('shift', 'up')

    // start up Charles system
    robot.typeString(`cd "C:\\Users\\2245760-MTS\\OneDrive - Church of Jesus Christ\\Desktop\\charles\\holly-master"`)
    robot.keyTap('enter')
    robot.typeString('cargo run --release')
    robot.keyTap('enter')


    // open new tab

    robot.keyToggle('control', 'down')
    robot.keyToggle('shift', 'down')
    robot.keyTap('t')
    robot.keyToggle('control', 'up')
    robot.keyToggle('shift', 'up')
    robot.keyToggle('shift', 'up')

    // start up referrals list
    robot.typeString(`cd "C:\\Users\\2245760-MTS\\OneDrive - Church of Jesus Christ\\Desktop\\charles\\referral_list-master"`)
    robot.keyTap('enter')
    robot.typeString('cargo run --release')
    robot.keyTap('enter')
    robot.keyTap('enter')
    robot.keyTap('down')
    robot.keyTap('enter')

    for (let i = 0; i <= 2; i++) {
        robot.keyTap('down')
    }

    robot.keyTap('enter')
    

    // head back to main
    //robot.keyToggle('control', 'down')
    //robot.keyToggle('shift', 'down')
    //robot.keyTap('tab')
    //robot.keyToggle('control', 'up')
    //robot.keyToggle('shift', 'up')
    //robot.keyToggle('shift', 'up')
    

    process.succeed('Charles has done his thing!')
}

// groovy loading indicator for monkeys 
function swirl(indicator, text) {
    indicator.text = text
}


function brain() {
    const loadCharles = loading(`summoning charles, don't touch any keys!'`).start()
    setTimeout(() => inflictCharles('./holly-master', 'cargo run --release', swirl, loadCharles), 1000)
}

brain()