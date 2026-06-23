<style scoped>
.hex-viewer {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  overflow-x: hidden;
  font-family: 'Courier New', Consolas, Monaco, monospace;
  font-size: 13px;
  background: #1e1e1e;
  color: #d4d4d4;
}

.hex-spacer {
  position: relative;
}

.hex-rows {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
}

.hex-row {
  display: flex;
  align-items: center;
  height: 20px;
  padding: 0 8px;
  white-space: nowrap;
}

.hex-offset {
  color: #6a9955;
  margin-right: 16px;
  user-select: text;
}

.hex-bytes {
  color: #d4d4d4;
  margin-right: 16px;
  user-select: text;
}

.hex-ascii {
  color: #9cdcfe;
  user-select: text;
}
</style>

<template>
  <div ref="container" class="hex-viewer" @scroll="onScroll">
    <div class="hex-spacer" :style="{ height: totalHeight + 'px' }">
      <div class="hex-rows" :style="{ transform: `translateY(${offsetY}px)` }">
        <div
            v-for="row in visibleRows"
            :key="row.startIndex"
            class="hex-row"
        >
          <span class="hex-offset">{{ row.offsetHex }}</span>
          <span class="hex-bytes">{{ row.hexString }}</span>
          <span class="hex-ascii">{{ row.asciiString }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
const ROW_HEIGHT = 20
const BUFFER_ROWS = 6
const OFFSET_DIGITS = 8

export default {
  name: 'HexViewer',
  props: {
    bytes: {
      type: Array,
      required: true
    }
  },
  data () {
    return {
      scrollTop: 0,
      containerHeight: 0,
      bytesPerRow: 16,
      charWidth: 8,
      resizeObserver: null
    }
  },
  computed: {
    totalRows () {
      if (this.bytesPerRow <= 0) return 0
      return Math.ceil(this.bytes.length / this.bytesPerRow)
    },
    totalHeight () {
      return this.totalRows * ROW_HEIGHT
    },
    startRow () {
      return Math.max(0, Math.floor(this.scrollTop / ROW_HEIGHT) - BUFFER_ROWS)
    },
    visibleRowCount () {
      const rowsInView = Math.ceil(this.containerHeight / ROW_HEIGHT)
      return rowsInView + BUFFER_ROWS * 2
    },
    offsetY () {
      return this.startRow * ROW_HEIGHT
    },
    visibleRows () {
      const rows = []
      const endRow = Math.min(this.totalRows, this.startRow + this.visibleRowCount)

      for (let r = this.startRow; r < endRow; r++) {
        const startIndex = r * this.bytesPerRow
        const endIndex = Math.min(this.bytes.length, startIndex + this.bytesPerRow)

        let hexParts = []
        let asciiParts = []

        for (let i = startIndex; i < endIndex; i++) {
          const byte = this.bytes[i]
          hexParts.push(byte.toString(16).padStart(2, '0'))
          asciiParts.push(byte >= 32 && byte <= 126 ? String.fromCharCode(byte) : '.')
        }

        // pad the last (partial) row so columns stay aligned
        const padCount = this.bytesPerRow - (endIndex - startIndex)
        for (let p = 0; p < padCount; p++) {
          hexParts.push('  ')
        }

        rows.push({
          startIndex,
          offsetHex: startIndex.toString(16).padStart(OFFSET_DIGITS, '0'),
          hexString: hexParts.join(' '),
          asciiString: asciiParts.join('')
        })
      }

      return rows
    }
  },
  mounted () {
    this.measureCharWidth()
    this.recalculateLayout()

    this.resizeObserver = new ResizeObserver(() => {
      this.recalculateLayout()
    })
    this.resizeObserver.observe(this.$refs.container)
  },
  beforeUnmount () {
    if (this.resizeObserver) {
      this.resizeObserver.disconnect()
    }
  },
  methods: {
    onScroll (e) {
      this.scrollTop = e.target.scrollTop
    },
    measureCharWidth () {
      const span = document.createElement('span')
      span.style.fontFamily = getComputedStyle(this.$refs.container).fontFamily
      span.style.fontSize = getComputedStyle(this.$refs.container).fontSize
      span.style.position = 'absolute'
      span.style.visibility = 'hidden'
      span.style.whiteSpace = 'pre'
      span.textContent = '00000000'
      document.body.appendChild(span)
      this.charWidth = span.getBoundingClientRect().width / 8
      document.body.removeChild(span)
    },
    recalculateLayout () {
      const container = this.$refs.container
      if (!container) return

      this.containerHeight = container.clientHeight

      const containerWidth = container.clientWidth
      const offsetColWidth = (OFFSET_DIGITS + 2) * this.charWidth // + gap
      const columnGap = 2 * this.charWidth // gap between hex/ascii columns
      const availableWidth = containerWidth - offsetColWidth - columnGap

      const widthPerByte = this.charWidth * 4
      let computed = Math.floor(availableWidth / widthPerByte)

      computed = Math.max(4, computed - (computed % 4))

      this.bytesPerRow = computed
    }
  }
}
</script>
