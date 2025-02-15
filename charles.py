import pexpect
import time

def printProgressBar(iteration, total, prefix='', suffix='', decimals=1, length=50, fill='█', printEnd="\r"):
    """
    Call in a loop to create a terminal progress bar
    """
    percent = ("{0:." + str(decimals) + "f}").format(100 * (iteration / float(total)))
    filledLength = int(length * iteration // total)
    bar = fill * filledLength + '-' * (length - filledLength)
    print(f'\r{prefix} |{bar}| {percent}% {suffix}', end=printEnd)
    if iteration == total:
        print()  # New line when complete

def update_progress_bar(prev, pos, total):
    for i in range(prev, pos):
        time.sleep(0.1)
        printProgressBar(i + 1, total, prefix='Progress:', suffix='Complete', length=50)

def main():
    print('Waking Charles up!')
    total_steps = 4  
    printProgressBar(0, total_steps, prefix='Progress:', suffix='Complete', length=50)
    
    update_progress_bar(0, 2, total_steps)  
    time.sleep(2)
    update_progress_bar(2, 4, total_steps)  

main()
