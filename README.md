# RAG Web Application

A sophisticated Retrieval-Augmented Generation (RAG) web application that combines Rust backend with Llama2 LLM, Next.js frontend, and advanced document processing techniques.

## 🚀 Features

- **RAG Implementation**
  - Advanced relevance scoring for accurate document retrieval
  - Smart document chunking for optimal processing
  - Rich metadata integration with stored documents
  - Vector similarity search using Qdrant

- **Backend (Rust)**
  - Llama2 model integration
  - High-performance document processing
  - RESTful API endpoints
  - Efficient memory management

- **Frontend (Next.js 13+)**
  - App Router architecture
  - TypeScript implementation
  - Modern responsive UI
  - Real-time query processing

- **Database Integration**
  - MongoDB for storing:
    - User queries
    - Historical prompts
    - Processing results
  - Qdrant for vector storage:
    - Document embeddings
    - Semantic search capabilities

## 🛠️ Technical Stack

### Backend
- Rust
- Llama2 LLM
- RESTful API
- Document processing pipeline

### Frontend
- Next.js 13+ (App Router)
- TypeScript
- TailwindCSS
- React Query

### Databases
- MongoDB
- Qdrant Vector Database

## 📋 Prerequisites

- Rust ^1.70
- Node.js ^18
- MongoDB ^6.0
- Qdrant latest version
- Llama2 model files

## 🔧 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/ChaiyawutTar/RAG-web.git
   cd RAG-web
   ```

2. **Backend Setup**
   ```bash
   cd backend
   cargo build
   cargo run
   ```

3. **Frontend Setup**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

4. **Database Setup**
   - Configure MongoDB connection
   - Set up Qdrant instance
   - Initialize vector collections

## 🏗️ Architecture

### RAG Pipeline
1. Document Processing
   - Chunking
   - Metadata extraction
   - Embedding generation

2. Storage Layer
   - Vector storage in Qdrant
   - Document metadata in MongoDB

3. Retrieval Process
   - Query embedding
   - Similarity search
   - Relevance scoring
   - Context assembly

## 💻 Usage

1. Upload documents through the web interface
2. Documents are automatically processed and stored
3. Query the system using natural language
4. Receive contextually relevant responses

## ⚙️ Configuration

Create a `.env` file with the following variables:

```env
MONGODB_URI=your_mongodb_uri
QDRANT_URL=your_qdrant_url
LLAMA2_MODEL_PATH=path_to_model
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## ✨ Acknowledgments

- Llama2 team for the language model
- Rust community for backend support
- Next.js team for frontend framework
- MongoDB and Qdrant teams for database solutions

---
**Note**: This is an active project under development. Features and documentation may be updated frequently.
