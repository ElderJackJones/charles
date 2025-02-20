import subprocess
import time
import pyautogui
import PySimpleGUI as sg

def watch_log_file(log_path):
    with open(log_path, "r") as file:
        file.seek(0, 2)  # Move to the end of the file
        while True:
            line = file.readline()
            if line:
                print(line.strip())
                if "DONE" in line:
                    pyautogui.write('q \n')
                    pyautogui.hotkey('ctrl', 'shift', 'w')
                    break
            else:
                time.sleep(1)  # Avoid high CPU usage



def main():
    # GUI code for username input (unchanged)
    lds_blue = "#023047"
    lds_white = "#ffffff"
    light_blue = "#8ecae6"

    sg.LOOK_AND_FEEL_TABLE['LDS'] = {
        'BACKGROUND': lds_white,
        'TEXT': lds_blue,
        'INPUT': lds_blue,
        'TEXT_INPUT': lds_white,
        'SCROLL': lds_white,
        'BUTTON': (lds_white, lds_blue),
        'PROGRESS': (lds_blue, lds_white),
        'BORDER': 1,
        'SLIDER_DEPTH': 0,
        'PROGRESS_DEPTH': 0
    }
    sg.theme('LDS')

    layout = [
        [sg.Text("What is your church username?", font=("Segoe UI", 14), justification='center', size=(40, 1))],
        [sg.Input(key='username', font=("Segoe UI", 10), size=(30, 1), justification='left', background_color=lds_white, text_color=lds_blue, border_width=2)],
        [sg.Button("Submit", font=("Segoe UI", 10), bind_return_key=True, button_color=(lds_blue, light_blue), border_width=2),
         sg.Button("Cancel", font=("Segoe UI", 10), button_color=(lds_blue, light_blue), border_width=2)]
    ]

    window = sg.Window("Church Account Login", layout, size=(600, 200), element_justification='center', finalize=True)

    username = None
    while True:
        event, values = window.read()
        if event in (sg.WIN_CLOSED, "Cancel"):
            return  # Exit on cancel
        if event == "Submit":
            username = values['username'].strip()
            if username:
                break
            else:
                sg.popup_error("Username cannot be empty!")  # Error message

    window.close()

    print("Username:", username)

    # Start the subprocesses
    holly = subprocess.Popen(
        ["powershell", "-NoExit", "-Command", "cd holly-master; cargo run --release"],
        creationflags=subprocess.CREATE_NEW_CONSOLE,
    )

    referral = subprocess.Popen(
        ["powershell", "-NoExit", "-Command", "cd referral_list-master; cargo run --release"],
        creationflags=subprocess.CREATE_NEW_CONSOLE,
    )

    # Open holly
    time.sleep(3)
    pyautogui.write(username + "\n", interval=0.1)
    pyautogui.press(['enter', 'enter', 'enter'])
    
    for i in range(3):
        pyautogui.press('down')

    time.sleep(15)
    pyautogui.press('enter')

    watch_log_file('referral_list-master\holly.log')

    holly.wait()
    referral.wait()



main()
