document.addEventListener('alpine:init', () => {
  Alpine.data('select2', () => ({
    optionsVisible: false,
    search: "",
    selected: {
      label: "",
      value: ""
    },
    clearSearch() {
      this.selected.value = '';
      this.selected.label = '';
    },
    setInitial(selected) {
      selectedOption = this.options.find((option) => {
        return option.value == selected
      });
      if (selectedOption != undefined) {
        this.selectOption(selectedOption);
      }
    },
    showOptions() {
      this.optionsVisible = true;
    },
    hideOptions() {
      this.optionsVisible = false;
    },
    selectOption(option) {
      this.selected = option;
      this.hideOptions();
      this.search = option.label
    },
    options: [],
    filteredOptions() {
      return this.options.filter((option, i) => {
        return option.label.toLowerCase().includes(this.search.toLowerCase());
      });
    },
    highlight(value) {
      var text = this.search.trim();
      if (text == '') {
        return value;
      }
      var query = new RegExp(`(${text})`, "ig");
      return value.replace(query, '<span class="font-extrabold text-blue-600">$1</span>');
    }
  }))
})
