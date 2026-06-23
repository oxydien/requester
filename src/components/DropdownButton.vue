
<style lang="scss" scoped>
.btn-dropdown {
  position: relative;
  display: inline-block;
}

.btn-dropdown-group {
  display: flex;
  align-items: stretch;

  .btn-dropdown-main {
    border-radius: var(--radius-md) 0 0 var(--radius-md);
    border-right: 1px solid rgba(0, 0, 0, 0.15);
  }

  .btn-dropdown-caret {
    border-radius: 0 var(--radius-md) var(--radius-md) 0;
    padding-left: var(--gap-sm);
    padding-right: var(--gap-sm);

    .arrow {
      transition: transform 0.2s ease;

      &.rotate {
        transform: rotate(180deg);
      }
    }
  }
}

.btn-dropdown.render-down .btn-dropdown-group {
  .btn-dropdown-main,
  .btn-dropdown-caret {
    &:focus-within,
    &:focus {
      outline: 0;
    }
  }
}

.options-wrapper {
  position: absolute;
  width: 100%;
  overflow: auto;
  z-index: 9;

  &.up {
    top: 0;
    transform: translateY(-99.999%);
    border-radius: var(--radius-md) var(--radius-md) 0 0;
  }

  &.down {
    border-radius: 0 0 var(--radius-md) var(--radius-md);
  }
}

.options {
  z-index: 10;
  max-height: 18.75rem;
  overflow-y: auto;
  box-shadow: var(--shadow-inset-sm), 0 0 0 0 transparent;
  min-width: 100%;

  .option {
    background-color: var(--color-button-bg);
    display: flex;
    align-items: center;
    padding: var(--gap-md);
    cursor: pointer;
    user-select: none;
    white-space: nowrap;

    &:hover {
      filter: brightness(0.85);
      transition: filter 0.2s ease-in-out;
    }

    &:focus {
      outline: 0;
      filter: brightness(0.85);
      transition: filter 0.2s ease-in-out;
    }

    &.primary-option {
      font-weight: bolder;
    }
  }
}

.options-enter-active,
.options-leave-active {
  transition: transform 0.2s ease;
}

.options-enter-from,
.options-leave-to {
  &.up {
    transform: translateY(99.999%);
  }

  &.down {
    transform: translateY(-99.999%);
  }
}

.options-enter-to,
.options-leave-from {
  &.up {
    transform: translateY(0%);
  }
}

:where(button) {
  background: none;
  color: var(--color-base);
}
</style>

<template>
  <div
      ref="dropdown"
      class="btn-dropdown"
      :class="{ 'render-up': renderUp, 'render-down': !renderUp }"
      @focusout="onBlur"
      @keydown.up.prevent="focusPreviousOption"
      @keydown.down.prevent="focusNextOption"
  >
    <div class="btn-dropdown-group">
      <button
          class="btn btn-dropdown-main"
          :class="btnClasses"
          :disabled="disabled"
          @click="triggerPrimary"
      >
        <slot>{{ selectedActionName }}</slot>
        <UnknownIcon v-if="!$slots.default && !selectedActionName" />
      </button>
      <button
          type="button"
          class="btn btn-dropdown-caret"
          :class="btnClasses"
          :disabled="disabled"
          tabindex="0"
          role="combobox"
          :aria-expanded="dropdownVisible"
          @click="toggleDropdown"
          @keydown.enter.prevent="toggleDropdown"
          @keydown.space.prevent="toggleDropdown"
      >
        <DropdownIcon class="arrow" :class="{ rotate: dropdownVisible }" />
      </button>
    </div>

    <div class="options-wrapper" :class="{ down: !renderUp, up: renderUp }">
      <transition name="options">
        <div
            v-show="dropdownVisible"
            class="options"
            role="listbox"
            :class="{ down: !renderUp, up: renderUp }"
        >
          <div
              v-for="(actionName, _) in actionNames"
              :key="actionName"
              ref="optionElements"
              tabindex="-1"
              role="option"
              class="option"
              :class="{ 'primary-option': actionName === primaryActionName }"
              @click="selectOption(actionName)"
              @keydown.enter.prevent="selectOption(actionName)"
              @keydown.space.prevent="selectOption(actionName)"
          >
            <label>{{ actionName }}</label>
          </div>
        </div>
      </transition>
    </div>
  </div>
</template>

<script setup>
import { UnknownIcon, DropdownIcon } from 'omorphia'
import { computed, ref } from 'vue'

const props = defineProps({
  // { actionName1: func, actionName2: func2, ... }
  // The first key in this object is treated as the primary action.
  actions: {
    type: Object,
    required: true,
  },
  color: {
    type: String,
    default: 'default',
  },
  large: {
    type: Boolean,
    default: false,
  },
  outline: {
    type: Boolean,
    default: false,
  },
  transparent: {
    type: Boolean,
    default: false,
  },
  hoverFilled: {
    type: Boolean,
    default: false,
  },
  hoverFilledOnly: {
    type: Boolean,
    default: false,
  },
  disabled: {
    type: Boolean,
    default: false,
  },
  renderUp: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['action'])

const actionNames = computed(() => Object.keys(props.actions))
const primaryActionName = computed(() => actionNames.value[0] || null)
const selectedActionName = computed(() => lastSelectedAction.value || primaryActionName)

const dropdownVisible = ref(false)
const focusedOptionIndex = ref(null)
const dropdown = ref(null)
const optionElements = ref(null)
const lastSelectedAction = ref(null)

const accentedButton = computed(() =>
    ['danger', 'primary', 'red', 'orange', 'green', 'blue', 'purple', 'gray'].includes(props.color)
)

const btnClasses = computed(() => ({
  'btn-large': props.large,
  'btn-danger': props.color === 'danger',
  'btn-primary': props.color === 'primary',
  'btn-secondary': props.color === 'secondary',
  'btn-highlight': props.color === 'highlight',
  'btn-red': props.color === 'red',
  'btn-orange': props.color === 'orange',
  'btn-green': props.color === 'green',
  'btn-blue': props.color === 'blue',
  'btn-purple': props.color === 'purple',
  'btn-gray': props.color === 'gray',
  'btn-transparent': props.transparent,
  'btn-hover-filled': props.hoverFilled,
  'btn-hover-filled-only': props.hoverFilledOnly,
  'btn-outline': props.outline,
  'color-accent-contrast': accentedButton.value,
  disabled: props.disabled,
}))

const runAction = (actionName) => {
  if (props.disabled) return
  const fn = props.actions[actionName]
  if (typeof fn === 'function') {
    fn()
  }
  lastSelectedAction.value = actionName;
  emit('action', actionName)
}

const triggerPrimary = () => {
  if (props.disabled || !primaryActionName.value) return
  runAction(primaryActionName.value)
}

const selectOption = (actionName) => {
  runAction(actionName)
  dropdownVisible.value = false
}

const toggleDropdown = () => {
  if (props.disabled) return
  dropdownVisible.value = !dropdownVisible.value
  if (dropdownVisible.value) {
    focusedOptionIndex.value = 0
  }
}

const onBlur = (event) => {
  if (!isChildOfDropdown(event.relatedTarget)) {
    dropdownVisible.value = false
  }
}

const isChildOfDropdown = (element) => {
  let currentNode = element
  while (currentNode) {
    if (currentNode === dropdown.value) {
      return true
    }
    currentNode = currentNode.parentNode
  }
  return false
}

const focusPreviousOption = () => {
  if (props.disabled) return
  if (!dropdownVisible.value) {
    toggleDropdown()
    return
  }
  focusedOptionIndex.value =
      (focusedOptionIndex.value + actionNames.value.length - 1) % actionNames.value.length
  optionElements.value[focusedOptionIndex.value].focus()
}

const focusNextOption = () => {
  if (props.disabled) return
  if (!dropdownVisible.value) {
    toggleDropdown()
    return
  }
  focusedOptionIndex.value = (focusedOptionIndex.value + 1) % actionNames.value.length
  optionElements.value[focusedOptionIndex.value].focus()
}
</script>
