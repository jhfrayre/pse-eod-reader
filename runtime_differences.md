## New request each time
### Downloading for February to December 2022 only
```
Downloading ran for 512.346 seconds.
```

## New request each time
### Downloading for January 2022 only

```bash
jan@jan-workstation:~/repositories/pse-eod-reader$ python3 src/download_pdf.py
Please enter the year of data to download
2022
PDF downloaded successfully at document/January 03, 2022-EOD.pdf
Failed to download https://documents.pse.com.ph/market_report/January 04, 2022-EOD.pdf. Status code: 404
PDF downloaded successfully at document/January 05, 2022-EOD.pdf
PDF downloaded successfully at document/January 06, 2022-EOD.pdf
PDF downloaded successfully at document/January 07, 2022-EOD.pdf
PDF downloaded successfully at document/January 10, 2022-EOD.pdf
PDF downloaded successfully at document/January 11, 2022-EOD.pdf
PDF downloaded successfully at document/January 12, 2022-EOD.pdf
PDF downloaded successfully at document/January 13, 2022-EOD.pdf
PDF downloaded successfully at document/January 14, 2022-EOD.pdf
PDF downloaded successfully at document/January 17, 2022-EOD.pdf
PDF downloaded successfully at document/January 18, 2022-EOD.pdf
PDF downloaded successfully at document/January 19, 2022-EOD.pdf
PDF downloaded successfully at document/January 20, 2022-EOD.pdf
PDF downloaded successfully at document/January 21, 2022-EOD.pdf
PDF downloaded successfully at document/January 24, 2022-EOD.pdf
PDF downloaded successfully at document/January 25, 2022-EOD.pdf
PDF downloaded successfully at document/January 26, 2022-EOD.pdf
PDF downloaded successfully at document/January 27, 2022-EOD.pdf
PDF downloaded successfully at document/January 28, 2022-EOD.pdf
PDF downloaded successfully at document/January 31, 2022-EOD.pdf
Downloading ran for 49.548 seconds.
```


## With session
### Downloading for January 2022 only

```bash
jan@jan-workstation:~/repositories/pse-eod-reader$ python3 src/download_pdf.py
Please enter the year of data to download
2022
PDF downloaded successfully at document/January 03, 2022-EOD.pdf
Failed to download https://documents.pse.com.ph/market_report/January 04, 2022-EOD.pdf. Status code: 404
PDF downloaded successfully at document/January 05, 2022-EOD.pdf
PDF downloaded successfully at document/January 06, 2022-EOD.pdf
PDF downloaded successfully at document/January 07, 2022-EOD.pdf
PDF downloaded successfully at document/January 10, 2022-EOD.pdf
PDF downloaded successfully at document/January 11, 2022-EOD.pdf
PDF downloaded successfully at document/January 12, 2022-EOD.pdf
PDF downloaded successfully at document/January 13, 2022-EOD.pdf
PDF downloaded successfully at document/January 14, 2022-EOD.pdf
PDF downloaded successfully at document/January 17, 2022-EOD.pdf
PDF downloaded successfully at document/January 18, 2022-EOD.pdf
PDF downloaded successfully at document/January 19, 2022-EOD.pdf
PDF downloaded successfully at document/January 20, 2022-EOD.pdf
PDF downloaded successfully at document/January 21, 2022-EOD.pdf
PDF downloaded successfully at document/January 24, 2022-EOD.pdf
PDF downloaded successfully at document/January 25, 2022-EOD.pdf
PDF downloaded successfully at document/January 26, 2022-EOD.pdf
PDF downloaded successfully at document/January 27, 2022-EOD.pdf
PDF downloaded successfully at document/January 28, 2022-EOD.pdf
PDF downloaded successfully at document/January 31, 2022-EOD.pdf
Downloading ran for 25.784 seconds.
jan@jan-workstation:~/repositories/pse-eod-reader$
```

## With session and pooling
### Downloading for January 2022 only

```bash
jan@jan-workstation:~/repositories/pse-eod-reader$ python3 src/download_pdf.py
Please enter the year of data to download: 2022
Use multiprocessing? (Faster but may encounter request timeouts) [Y/n]: Y
Running with multiprocessing: ON
Failed to download https://documents.pse.com.ph/market_report/January 04, 2022-EOD.pdf. Status code: 404
PDF downloaded successfully at document/January 10, 2022-EOD.pdf
PDF downloaded successfully at document/January 07, 2022-EOD.pdf
PDF downloaded successfully at document/January 06, 2022-EOD.pdf
PDF downloaded successfully at document/January 11, 2022-EOD.pdf
PDF downloaded successfully at document/January 03, 2022-EOD.pdf
PDF downloaded successfully at document/January 12, 2022-EOD.pdf
PDF downloaded successfully at document/January 05, 2022-EOD.pdf
PDF downloaded successfully at document/January 19, 2022-EOD.pdf
PDF downloaded successfully at document/January 18, 2022-EOD.pdf
PDF downloaded successfully at document/January 14, 2022-EOD.pdf
PDF downloaded successfully at document/January 13, 2022-EOD.pdf
PDF downloaded successfully at document/January 17, 2022-EOD.pdf
PDF downloaded successfully at document/January 21, 2022-EOD.pdf
PDF downloaded successfully at document/January 25, 2022-EOD.pdf
PDF downloaded successfully at document/January 26, 2022-EOD.pdf
PDF downloaded successfully at document/January 20, 2022-EOD.pdf
PDF downloaded successfully at document/January 24, 2022-EOD.pdf
PDF downloaded successfully at document/January 28, 2022-EOD.pdf
PDF downloaded successfully at document/January 31, 2022-EOD.pdf
PDF downloaded successfully at document/January 27, 2022-EOD.pdf
Downloading ran for 3.953 seconds.
jan@jan-workstation:~/repositories/pse-eod-reader$
```


## With session and pooling
### Downloading for January to December 2022
```bash
jan@jan-workstation:~/repositories/pse-eod-reader$ python3 src/download_pdf.py
Please enter the year of data to download: 2022
Use multiprocessing? (Faster but may encounter request timeouts) [Y/n]: Y
Running with multiprocessing: ON
Failed to download https://documents.pse.com.ph/market_report/January 04, 2022-EOD.pdf. Status code: 404
PDF downloaded successfully at document/January 07, 2022-EOD.pdf
PDF downloaded successfully at document/January 05, 2022-EOD.pdf
PDF downloaded successfully at document/January 03, 2022-EOD.pdf
PDF downloaded successfully at document/January 11, 2022-EOD.pdf
PDF downloaded successfully at document/January 13, 2022-EOD.pdf
PDF downloaded successfully at document/January 12, 2022-EOD.pdf
PDF downloaded successfully at document/January 06, 2022-EOD.pdf
PDF downloaded successfully at document/January 14, 2022-EOD.pdf
PDF downloaded successfully at document/January 20, 2022-EOD.pdf
PDF downloaded successfully at document/January 21, 2022-EOD.pdf
PDF downloaded successfully at document/January 10, 2022-EOD.pdf
PDF downloaded successfully at document/January 26, 2022-EOD.pdf
PDF downloaded successfully at document/January 27, 2022-EOD.pdf
PDF downloaded successfully at document/January 18, 2022-EOD.pdf
PDF downloaded successfully at document/January 17, 2022-EOD.pdf
PDF downloaded successfully at document/January 19, 2022-EOD.pdf
PDF downloaded successfully at document/January 24, 2022-EOD.pdf
Failed to download https://documents.pse.com.ph/market_report/February 01, 2022-EOD.pdf. Status code: 404
PDF downloaded successfully at document/February 07, 2022-EOD.pdf
PDF downloaded successfully at document/January 25, 2022-EOD.pdf
PDF downloaded successfully at document/February 09, 2022-EOD.pdf
PDF downloaded successfully at document/January 31, 2022-EOD.pdf
PDF downloaded successfully at document/January 28, 2022-EOD.pdf
PDF downloaded successfully at document/February 14, 2022-EOD.pdf
Failed to download https://documents.pse.com.ph/market_report/December 26, 2022-EOD.pdf. Status code: 404
PDF downloaded successfully at document/December 19, 2022-EOD.pdf
Failed to download https://documents.pse.com.ph/market_report/December 29, 2022-EOD.pdf. Status code: 404
PDF downloaded successfully at document/December 22, 2022-EOD.pdf
PDF downloaded successfully at document/December 27, 2022-EOD.pdf
Failed to download https://documents.pse.com.ph/market_report/December 30, 2022-EOD.pdf. Status code: 404
PDF downloaded successfully at document/December 28, 2022-EOD.pdf
PDF downloaded successfully at document/December 23, 2022-EOD.pdf
Downloading ran for 41.404 seconds.
```
