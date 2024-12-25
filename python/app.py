import requests
from bs4 import BeautifulSoup
from flask import Flask, jsonify
import time

app = Flask(__name__)

# Function to fetch and parse the HTML page
def fetch_page(url):
    response = requests.get(url)
    return response.text

def parse_page(html):
    soup = BeautifulSoup(html, 'html.parser')
    # Extract book titles (adjust based on actual website structure)
    titles = [title.text.strip() for title in soup.find_all('h3')]  # Example for books
    return titles

@app.route('/scrape', methods=['GET'])
def scrape():
    start_time = time.time()  # Start the timer

    base_url = "http://books.toscrape.com/catalogue/page-"
    all_titles = []
    page = 1

    while True:
        # Build the URL for each page
        url = f"{base_url}{page}.html"
        html = fetch_page(url)
        titles = parse_page(html)

        if not titles:
            break  # Stop if no titles are found on the current page
        
        all_titles.extend(titles)  # Add titles from the current page to the list
        page += 1  # Move to the next page

    end_time = time.time()  # End the timer
    elapsed_time = end_time - start_time  # Calculate the elapsed time

    print(f"Time taken: {elapsed_time:.2f} seconds")  # Print the elapsed time

    return jsonify({'titles': all_titles, 'time_taken': f"{elapsed_time:.2f} seconds"})

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=5000)
