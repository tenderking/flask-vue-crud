from flask import Flask, jsonify, request
from flask_cors import CORS
import uuid


# configuration
DEBUG = False

# some books data
BOOKS = [
    {
        'id': uuid.uuid4().hex,
        'title': 'On the Road',
        'author': 'Jack Kerouac',
        'read': True
    },
    {
        'id': uuid.uuid4().hex,
        'title': 'Harry Potter and the Philosopher\'s Stone',
        'author': 'J. K. Rowling',
        'read': False
    },
    {
        'id': uuid.uuid4().hex,
        'title': 'Green Eggs and Ham',
        'author': 'Dr. Seuss',
        'read': True
    }
]


# instantiate the app
app = Flask(__name__)

CORS(app)

# GET and POST methods


@app.route('/books', methods=['GET', 'POST'])
def all_books():
    response_object = {'status': 'success'}

    if request.method == 'POST':
        post_data = request.get_json()
        BOOKS.append({
            'id': uuid.uuid4().hex,
            'title': post_data.get('title'),
            'author': post_data.get('author'),
            'read': post_data.get('read')
        })
        response_object['message'] = 'Book added!'    
    else:
        response_object['books'] = BOOKS

    return jsonify(response_object)

# PUT method


@app.route('/books/<book_id>', methods=["PUT", "DELETE"])
def single_book(book_id):
    response_object = {'status': 'success'}

    if request.method == 'PUT':
        print('id is' + book_id)
        post_data=request.get_json() 
              
        for book in BOOKS:            
            print(book)           
            if book['id'] == book_id:
                BOOKS.remove(book)
                BOOKS.append({
                'id': uuid.uuid4().hex,
                'title': post_data.get('title'),
                'author': post_data.get('author'),
                'read': post_data.get('read')
                })
                response_object['message'] = 'Book updated!'
    else:  response_object['status'] = 400

    if request.method == 'DELETE':
        print('id is ' + book_id)

        for book in BOOKS:            
            if book['id'] == book_id:
                print("matching id")
                BOOKS.remove(book)
                response_object['message'] = 'Book found'

    else:  response_object['status'] =400  

    return jsonify(response_object)


def remove_book(book_id):
    for book in BOOKS:
        print(book['id'])
        if book['id'] == book_id:
            print("matching id")            
            BOOKS.remove(book)
            return True
      
        return False
      


if __name__ == '__main__':
    app.run()
