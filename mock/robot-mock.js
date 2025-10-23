// robot-mock.js — minimal mock of the Opentrons robot HTTP API (port 31950)
const express = require('express')
const multer = require('multer')
const crypto = require('crypto')

const app = express()
const upload = multer()
app.use(express.json())

// Health endpoint
app.get('/health', (_req, res) => {
  res.json({ status: 'ok', robotServer: 'mock', api: 'v2' })
})

// Protocol upload
app.post('/protocols', upload.any(), (req, res) => {
  // Expect multipart field 'files'
  const file = (req.files || [])[0]
  if (!file) return res.status(400).json({ error: 'no file uploaded (field "files")' })
  const id = crypto.randomUUID()
  console.log(`MOCK: received protocol (${file.originalname || 'protocol.py'}), id=${id}`)
  res.json({ data: { id } })
})

// Create run from protocol
app.post('/runs', (req, res) => {
  const protocolId = req.body?.data?.protocolId
  if (!protocolId) return res.status(400).json({ error: 'protocolId required' })
  const runId = crypto.randomUUID()
  console.log(`MOCK: created run ${runId} for protocol ${protocolId}`)
  res.json({ data: { id: runId, protocolId } })
})

// Run actions (play/pause/stop)
app.post('/runs/:id/actions', (req, res) => {
  const { id } = req.params
  const action = req.body?.data?.actionType
  console.log(`MOCK: run ${id} action ${action}`)
  res.json({ data: { id: crypto.randomUUID(), actionType: action } })
})

const PORT = 31950
app.listen(PORT, () => console.log(`MOCK robot listening on http://localhost:${PORT}`))