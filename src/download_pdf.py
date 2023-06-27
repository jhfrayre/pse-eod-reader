## This is a simple script to download the EOD PDF files from the PSE website.

import os.path
from multiprocessing.pool import Pool
import pandas as pd
import requests
import time

def download_year(start_date, end_date, multiprocess):
    start_of_year = pd.Timestamp(start_date)
    business_offset = pd.tseries.offsets.BusinessDay(n = 1)
    end_of_year = pd.Timestamp(end_date)
    date = start_of_year
    session = requests.Session()
    url_list = []
    pool = Pool(processes = 8)
    while date < end_of_year:

        if (date > end_of_year):
            break
        filename = date.strftime('%B %d, %Y') + "-EOD.pdf"
        pdf_url = "https://documents.pse.com.ph/market_report/" + filename
        pdf_destination = "document/" + filename

        if (os.path.isfile(pdf_destination) == False):
            if (multiprocess):
                url_list.append((session, pdf_url, pdf_destination))
            else:
                download_pdf(session, pdf_url, pdf_destination)

        date = date + business_offset

    if (multiprocess):
        # execute tasks in order, process results out of order
        pool.imap_unordered(download_pdf_list, url_list)
        pool.close()
        pool.join()

def download_pdf_list(args):
    session, url, destination = args
    download_pdf(session, url, destination)

def download_pdf(session, url, destination):
    response = session.get(url)
    if response.status_code == 200:
        with open(destination, 'wb') as file:
            file.write(response.content)
        print(f"PDF downloaded successfully at {destination}")
    else:
        print(f"Failed to download {url}. Status code: {response.status_code}")

start_date = input("Please enter the start date of data to download e.g. `2022-01-01`: ")
end_date = input("Please enter the end date of data to download e.g. `2022-12-31`: ")
multiprocess = input("Use multiprocessing? (Faster but may encounter request timeouts) [Y/n]: ").strip() == "Y"

print("Running with multiprocessing: " + ("ON" if multiprocess else "OFF"))
start_time = time.time()
download_year(start_date, end_date, multiprocess)
end_time = time.time()
print("Downloading ran for " + str(round(end_time - start_time, 3)) + " seconds.")
