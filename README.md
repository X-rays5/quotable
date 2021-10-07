# Quotable
this is a reimplementation of [quotable](https://github.com/lukePeavey/quotable) as a cloudflare worker using rust wasm

## API Reference

### Error handling
When a request returns a non 2xx status code. The error will be sent in plain text.

### Get random quote

Returns a single random quote from the database

```HTTP
GET /random
```

#### Response

```ts
{
  _id: string
  // The quotation text
  content: string
  // The full name of the author
  author: string
  authorId: string
  // The `slug` of the quote author
  authorSlug: string
  // The length of quote (number of characters)
  length: number
  // An array of tag names for this quote
  tags: string[]
}
```

### List Quotes

Get all quotes matching a given query. By default, this will return a paginated list of all quotes, sorted by `_id`. Quotes can also be filter by author, tag, and length.

```HTTP
GET /quotes
```

#### Response

```ts
{
  // The total amount of quotes
  count: number
  // The array of quotes
  results: Array<{
    _id: string
    // The quotation text
    content: string
    // The full name of the author
    author: string
    // The `slug` of the quote author
    authorSlug: string
    // The length of quote (number of characters)
    length: number
    // An array of tag names for this quote
    tags: string[]
  }>
}
```

### Get Quote By ID

Get a quote by its ID

```HTTP
GET /quotes/:id
```

#### Response

```ts
{
  _id: string
  // The quotation text
  content: string
  // The full name of the author
  author: string
  // The length of quote (number of characters)
  length: number
  // An array of tag names for this quote
  tags: string[]
}
```

### List Authors

Get all authors matching the given query. This endpoint can be used to list authors, with several options for sorting and filter. It can also be used to get author details for one or more specific authors, using the author slug or ids.

```HTTP
GET /authors
```

#### Response

```ts
{
  // The total amount of authors
  count: number
  // The array of authors
  results: Array<{
    // A unique id for this author
    _id: string
    // A brief, one paragraph bio of the author. Source: wiki API
    bio: string
    // A one-line description of the author. Typically it is the person's primary
    // occupation or what they are know for.
    description: string
    // The link to the author's wikipedia page or official website
    link: string
    // The authors full name
    name: string
    // A slug is a URL-friendly ID derived from the authors name. It can be used as
    slug: string
    // The number of quotes by this author
    quoteCount: string
  }>
}
```

### Get Author By ID

Get details about a specific author by `_id`.

```HTTP
GET /authors/:id
```

#### Response

```ts
{
  // A unique id for this author
  _id: string
  // A brief, one paragraph bio of the author. Source wiki API.
  bio: string
  // A one-line description of the author.
  description: string
  // The link to the author's wikipedia page or official website
  link: string
  // The authors full name
  name: string
  // A slug is a URL-friendly ID derived from the authors name. It can be used as
  slug: string
  // The number of quotes by this author
  quoteCount: string
}
```

### List Tags

```HTTP
GET /tags
```

#### Response

```ts
{
  // The number of all tags by this request
  count: number
  // The array of tags
  results: Array<{
    _id: string
    name: string
  }>
}
```

## Usage

Get a random quote (fetch)

```js
fetch('https://quotable.x-rays5.workers.dev/random')
  .then(response => response.json())
  .then(data => {
    console.log(`${data.content} —${data.author}`)
  })
```

Get a random quote (async/await)

```js
async function randomQuote() {
  const response = await fetch('https://quotable.x-rays5.workers.dev/random')
  const data = await response.json()
  console.log(`${data.content} —${data.author}`)
}
randomQuote()
```

Get a random quote (JQuery)

```js
$.getJSON('https://quotable.x-rays5.workers.dev/random', function (data) {
  console.log(`${data.content} —${data.author}`)
})
```
